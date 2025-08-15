#include "AppController.h"
#include "common.h"
#include "include/bed.h"
#include <QSystemTrayIcon>
#include <QtLogging>

AppController::AppController() = default;

void AppController::init() {
    if (Controller::isInit()) {
        qFatal() << "Controller have already been initialized!";
    }
    Controller::init(new AppController);
    auto *ctrl = AppController::get();
    connect(ctrl, &AppController::error, ctrl, &AppController::handleError,
            qontrol::UNIQUE);
}

auto AppController::get() -> AppController * {
    auto *ctrl = Controller::get();
    auto *controller = dynamic_cast<AppController *>(ctrl);
    return controller;
}

void AppController::initController() {
    auto ctrl = init_controller();
    m_rust_controller = std::make_optional(std::move(ctrl));
}

void AppController::initState() {
    init_rust_logger(LogLevel::Debug);

    m_tray_icon = new QSystemTrayIcon;
    m_tray_icon->setIcon(QIcon::fromTheme("dialog-information"));
    m_tray_icon->setVisible(true);

    // init the timer that poll notifications
    m_notif_timer = new QTimer;
    connect(m_notif_timer, &QTimer::timeout, this, &AppController::poll);
    m_notif_timer->start(100);
}

void AppController::osMessage(QString title, QString msg, int delay) { // NOLINT
    m_tray_icon->showMessage(title, msg, QSystemTrayIcon::NoIcon, delay);
}

void AppController::osInfo(QString title, QString msg, int delay) { // NOLINT
    m_tray_icon->showMessage(title, msg, QSystemTrayIcon::Information, delay);
}

void AppController::osWarning(QString title, QString msg, int delay) { // NOLINT
    m_tray_icon->showMessage(title, msg, QSystemTrayIcon::Warning, delay);
}

void AppController::osCritical(QString title, QString msg,
                               int delay) { // NOLINT
    m_tray_icon->showMessage(title, msg, QSystemTrayIcon::Critical, delay);
}

void AppController::stop() {
    m_tray_icon->hide();
    m_tray_icon->deleteLater();
}

void AppController::poll() {
    if (m_rust_controller.has_value()) {
        pollNotif();
        pollError();
    }
};

void AppController::pollNotif() {
    if (!m_rust_controller.has_value()) {
        return;
    }
    while (true) {
        auto notif = m_rust_controller.value()->poll();
        if (notif != Notification::None) {
            log_info(notif_to_string(notif));
        }
        RustScreen screen;
        switch (notif) {
        case Notification::UpdateDecrypt:
            screen = m_rust_controller.value()->decrypt_screen();
            emit updateDecrypt(screen);
            break;
        case Notification::UpdateEncrypt:
            screen = m_rust_controller.value()->encrypt_screen();
            emit updateEncrypt(screen);
            break;
        case Notification::None:
        default:
            return;
        }
    }
}

void AppController::pollError() {
    while (true) {
        auto err = m_rust_controller.value()->error();
        if (err.empty()) {
            return;
        }
        log_info(err);
        auto qerror = QString(err.c_str());
        emit error(qerror);
    }
}

void AppController::tryDecrypt() {
    if (m_rust_controller.has_value()) {
        m_rust_controller.value()->decrypt().try_decrypt();
    }
}

void AppController::decryptAddKey() {
    if (m_rust_controller.has_value()) {
        m_rust_controller.value()->decrypt().add_xpub();
    }
}

void AppController::decryptEditKey(size_t index, const QString &txt) {
    if (m_rust_controller.has_value()) {
        auto rTxt = rust::String(txt.toStdString());
        m_rust_controller.value()->decrypt().edit_xpub(index, rTxt);
    }
}

void AppController::decryptDeleteKey(size_t index) {
    if (m_rust_controller.has_value()) {
        m_rust_controller.value()->decrypt().remove_xpub(index);
    }
}

auto AppController::isKeyValid(const QString &key) -> bool {
    auto rTxt = rust::String(key.toStdString());
    return is_xpub_valid(rTxt);
}

void AppController::decryptSelectKey(size_t index, bool selected) {
    if (m_rust_controller.has_value()) {
        return m_rust_controller.value()->decrypt().set_selected(index,
                                                                 selected);
    }
}

void AppController::decryptSave(const QString &path) {
    auto rPath = rust::String(path.toStdString());
    if (m_rust_controller.has_value()) {
        m_rust_controller.value()->decrypt().save(rPath);
    }
}

void AppController::setMode(Mode mode) {
    m_mode = mode;
}

void AppController::tabChanged(int index) {
    switch (index) {
    case 0:
        setMode(Mode::Decrypt);
        break;
    case 1:
        setMode(Mode::Encrypt);
        break;
    }
}

void AppController::dropped(const QString &file_path) {
    if (m_rust_controller.has_value()) {
        rust::String rPath(file_path.toStdString());
        m_rust_controller.value()->drag_n_drop(rPath, m_mode);
    }
}

void AppController::decryptReset() {
    if (m_rust_controller.has_value()) {
        m_rust_controller.value()->decrypt().reset();
    }
}

void AppController::handleError(const QString &error) {
    auto *modal = new qontrol::Modal("Error", error);
    AppController::execModal(modal);
}

auto AppController::isDescriptorValid(const QString &key) -> bool {
    auto rTxt = rust::String(key.toStdString());
    return is_descriptor_valid(rTxt);
}

void AppController::encryptEditKey(size_t index, const QString &txt) {
    if (m_rust_controller.has_value()) {
        m_rust_controller.value()->encrypt().edit_xpub(
            index, rust::String(txt.toStdString()));
    }
}

void AppController::encryptAddKey() {
    if (m_rust_controller.has_value()) {
        m_rust_controller.value()->encrypt().add_xpub();
    }
}

void AppController::encryptDeleteKey(size_t index) {
    if (m_rust_controller.has_value()) {
        m_rust_controller.value()->encrypt().remove_xpub(index);
    }
}

void AppController::encryptSelectKey(size_t index, bool selected) {
    if (m_rust_controller.has_value()) {
        m_rust_controller.value()->encrypt().set_selected(index, selected);
    }
}

void AppController::encryptReset() {
    if (m_rust_controller.has_value()) {
        m_rust_controller.value()->encrypt().reset();
    }
}

void AppController::encryptSetDescriptor(const QString &txt) {
    if (m_rust_controller.has_value()) {
        auto rTxt = rust::String(txt.toStdString());
        m_rust_controller.value()->encrypt().set_descriptor(rTxt);
    }
}

void AppController::tryEncrypt() {
    if (m_rust_controller.has_value()) {
        m_rust_controller.value()->encrypt().try_encrypt();
    }
}

void AppController::encryptSave(const QString &path) {
    if (m_rust_controller.has_value()) {
        auto rPath = rust::String(path.toStdString());
        m_rust_controller.value()->encrypt().save(rPath);
    }
}
