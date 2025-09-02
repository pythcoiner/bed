#include "Decrypt.h"
#include "AppController.h"
#include "Column.h"
#include "Row.h"
#include "common.h"
#include "include/bed.h"
#include <QCheckBox>
#include <QLabel>
#include <QPushButton>
#include <QTextEdit>

namespace screen {

Decrypt::Decrypt(QWidget *parent) {
    this->setParent(parent);
    this->init();
    this->view();
    this->doConnect();
}

Decrypt::~Decrypt() = default;

void Decrypt::init() {

    m_device_header = new QLabel("Hardware devices:");
    m_device = new QLabel(devices(0));
    m_key_header = new QLabel("Keys:");

    m_add_key = new QPushButton("Add key");
    m_add_key->setFixedWidth(150);

    m_data_header = new QLabel("Encrypted backup:");

    m_data = new QTextEdit("");
    m_data->setFixedWidth(500);
    m_data->setFixedHeight(100);
    m_data->setAcceptDrops(false);

    m_decrypt_btn = new QPushButton("Decrypt");
    m_decrypt_btn->setFixedWidth(BTN_WIDTH);
    m_decrypt_btn->setEnabled(false);

    m_save_btn = new QPushButton("Save");
    m_save_btn->setFixedWidth(BTN_WIDTH);
    m_save_btn->setVisible(false);

    m_copy_btn = new QPushButton("Copy");
    m_copy_btn->setFixedWidth(BTN_WIDTH);
    m_copy_btn->setVisible(false);

    m_reset_btn = new QPushButton("Reset");
    m_reset_btn->setFixedWidth(BTN_WIDTH);
    m_reset_btn->setVisible(true);
    m_reset_btn->setEnabled(false);
}

void Decrypt::doConnect() {
    auto *ctrl = AppController::get();
    // AppController => Decrypt
    connect(ctrl, &AppController::updateDecrypt, this, &Decrypt::update,
            qontrol::UNIQUE);
    // Widget => Decrypt
    connect(m_save_btn, &QPushButton::clicked, this, &Decrypt::onSelectFile,
            qontrol::UNIQUE);
    connect(m_copy_btn, &QPushButton::clicked, this, &Decrypt::onCopy,
            qontrol::UNIQUE);
    // // Decrypt => AppController
    connect(m_add_key, &QPushButton::clicked, ctrl,
            &AppController::decryptAddKey, qontrol::UNIQUE);
    connect(m_decrypt_btn, &QPushButton::clicked, ctrl,
            &AppController::tryDecrypt, qontrol::UNIQUE);
    connect(this, &Decrypt::deleteKey, ctrl, &AppController::decryptDeleteKey,
            qontrol::UNIQUE);
    connect(this, &Decrypt::editKey, ctrl, &AppController::decryptEditKey,
            qontrol::UNIQUE);
    connect(this, &Decrypt::selectKey, ctrl, &AppController::decryptSelectKey,
            qontrol::UNIQUE);
    connect(m_reset_btn, &QPushButton::clicked, ctrl,
            &AppController::decryptReset, qontrol::UNIQUE);
}

void Decrypt::view() {
    auto *btn = (new qontrol::Row)
                    ->pushSpacer()
                    ->push(m_decrypt_btn)
                    ->push(m_copy_btn)
                    ->pushSpacer(H_SPACER)
                    ->push(m_save_btn)
                    ->pushSpacer(H_SPACER)
                    ->push(m_reset_btn)
                    ->pushSpacer();
    auto
        *addK = (new qontrol::Row)->pushSpacer()->push(m_add_key)->pushSpacer();

    auto *keys = (new qontrol::Column);
    for (auto *k : m_keys) {
        keys->push(k)->pushSpacer(V_SPACER);
    }

    delete m_scroll;
    m_scroll = scrollable_v(keys);

    auto *col = (new qontrol::Column)
                    ->pushSpacer()
                    ->push(m_device_header)
                    ->pushSpacer(V_SPACER)
                    ->push(m_device)
                    ->pushSpacer(V_SPACER)
                    ->push(m_key_header)
                    ->pushSpacer(V_SPACER)
                    ->push(m_scroll)
                    ->push(addK)
                    ->pushSpacer(V_SPACER)
                    ->push(m_data_header)
                    ->pushSpacer(V_SPACER)
                    ->push(m_data)
                    ->pushSpacer(V_SPACER)
                    ->push(btn)
                    ->pushSpacer();

    auto *row = (new qontrol::Row)->pushSpacer()->push(col)->pushSpacer();

    auto *oldWidget = m_widget;
    m_widget = margin(row);
    delete oldWidget;

    delete this->layout();
    this->setLayout(m_widget->layout());
}

auto Decrypt::decryptBtn() -> QPushButton * {
    return m_decrypt_btn;
}

auto Decrypt::widget() -> QWidget * {
    return m_widget;
}

void Decrypt::update(RustScreen screen) {

    // devices
    m_device->setText(devices(screen.devices));

    // expand m_keys
    if (screen.keys.size() > m_keys.size()) {
        for (auto i = m_keys.size(); i < screen.keys.size(); i++) {
            auto *newKey = new widget::Key;
            auto *delBtn = newKey->delBtn();
            auto *input = newKey->input();
            auto *checkbox = newKey->checkbox();
            checkbox->setEnabled(false);
            connect(delBtn, &QPushButton::clicked, this,
                    [this, i]() { this->deleteKey(i); });
            connect(input, &QLineEdit::editingFinished, this,
                    [this, input, i]() {
                        auto txt = input->text();
                        this->editKey(i, txt);
                    });
            connect(checkbox, &QCheckBox::toggled, this, [this, checkbox, i]() {
                auto selected = checkbox->isChecked();
                emit selectKey(i, selected);
            });
            m_keys.append(newKey);
        }
    }
    // or retract m_keys
    while (screen.keys.size() < m_keys.size()) {
        auto *last = m_keys.last();
        delete last;
        m_keys.removeLast();
    }
    m_decrypted = !screen.descriptor.empty();

    // update all keys
    for (size_t i = 0; i < screen.keys.size(); i++) {
        auto txt = QString(screen.keys.at(i).c_str());
        if (txt != m_keys.at(i)->text()) {
            m_keys.at(i)->setText(txt, screen.valid.at(i));
        }
        m_keys.at(i)->setChecked(screen.selected.at(i));
        m_keys.at(i)->setEnabled(!m_decrypted);
        auto selected = screen.selected.at(i);
        m_keys.at(i)->setChecked(selected);
    }

    // update data
    QString txt;
    if (m_decrypted) {
        m_data_header->setText("Descriptor:");
        txt = QString(screen.descriptor.c_str());
        m_decrypt_btn->setVisible(false);
        m_save_btn->setVisible(true);
        m_copy_btn->setVisible(true);
        m_data->setEnabled(true);
    } else {
        m_decrypt_btn->setVisible(true);
        m_save_btn->setVisible(false);
        m_copy_btn->setVisible(false);
        m_data_header->setText("Encrypted descriptor:");
        m_data->setEnabled(false);
        if (screen.ciphertext.empty()) {
            txt = "";
        } else {
            txt = "[encrypted payload]";
        }
    }

    if (screen.ciphertext.empty() && screen.descriptor.empty() &&
        screen.keys.empty()) {
        m_reset_btn->setEnabled(false);
    } else {
        m_reset_btn->setEnabled(true);
    }

    if (screen.ciphertext.empty()) {
        m_decrypt_btn->setEnabled(false);
    } else {
        m_decrypt_btn->setEnabled(true);
    }

    m_data->setText(txt);

    view();
}

void Decrypt::onSelectFile() {
    auto *dialog = new QFileDialog(this);
    dialog->setAcceptMode(QFileDialog::AcceptMode::AcceptSave);
    dialog->selectFile("bed.descriptor");
    auto *ctrl = AppController::get();
    connect(dialog, &QFileDialog::fileSelected, ctrl,
            &AppController::decryptSave, qontrol::UNIQUE);
    AppController::execModal(dialog);
}

void Decrypt::onCopy() {
    if (!m_decrypted)
        return;
    auto *clipboard = QGuiApplication::clipboard();
    clipboard->setText(m_data->toPlainText());
}

auto Decrypt::devices(size_t count) -> QString {
    if (count == 0) {
        return "0 connected, connect a device for "
               "automatically fetch decryption keys.";
    } else if (count == 1) {
        return QString::number(count) + "device connected.";
    } else {
        return QString::number(count) + "devices connected.";
    }
}
} // namespace screen
