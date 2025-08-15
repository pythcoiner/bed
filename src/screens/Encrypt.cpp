#include "Encrypt.h"
#include "AppController.h"
#include "Column.h"
#include "Row.h"
#include "common.h"
#include "include/bed.h"
#include <QCheckBox>
#include <QLabel>
#include <QPushButton>
#include <QTextEdit>
#include <QtLogging>

namespace screen {

Encrypt::Encrypt(QWidget *parent) {
    this->setParent(parent);
    this->init();
    this->view();
    this->doConnect();
}

Encrypt::~Encrypt() = default;

void Encrypt::init() {

    m_key_header = new QLabel("Keys:");

    m_add_key = new QPushButton("Add key");
    m_add_key->setFixedWidth(150);

    m_data_header = new QLabel("Descriptor:");

    m_data = new QTextEdit("");
    m_data->setFixedWidth(500);
    m_data->setFixedHeight(100);
    m_data->setAcceptDrops(false);

    m_encrypt_btn = new QPushButton("Encrypt");
    m_encrypt_btn->setFixedWidth(BTN_WIDTH);
    m_encrypt_btn->setEnabled(false);

    m_save_btn = new QPushButton("Save");
    m_save_btn->setFixedWidth(BTN_WIDTH);
    m_save_btn->setVisible(false);

    m_reset_btn = new QPushButton("Reset");
    m_reset_btn->setFixedWidth(BTN_WIDTH);
    m_reset_btn->setVisible(true);
    m_reset_btn->setEnabled(false);
}

void Encrypt::doConnect() {
    auto *ctrl = AppController::get();
    // AppController => Encrypt
    connect(ctrl, &AppController::updateEncrypt, this, &Encrypt::update,
            qontrol::UNIQUE);
    // Widget => Encrypt
    connect(m_save_btn, &QPushButton::clicked, this, &Encrypt::onSelectFile,
            qontrol::UNIQUE);
    // Encrypt => AppController
    connect(this, &Encrypt::onFileSelected, ctrl, &AppController::encryptSave,
            qontrol::UNIQUE);
    connect(m_add_key, &QPushButton::clicked, ctrl,
            &AppController::encryptAddKey, qontrol::UNIQUE);
    connect(m_encrypt_btn, &QPushButton::clicked, ctrl,
            &AppController::tryEncrypt, qontrol::UNIQUE);
    connect(this, &Encrypt::deleteKey, ctrl, &AppController::encryptDeleteKey,
            qontrol::UNIQUE);
    connect(this, &Encrypt::editKey, ctrl, &AppController::encryptEditKey,
            qontrol::UNIQUE);
    connect(this, &Encrypt::selectKey, ctrl, &AppController::encryptSelectKey,
            qontrol::UNIQUE);
    connect(m_reset_btn, &QPushButton::clicked, ctrl,
            &AppController::encryptReset, qontrol::UNIQUE);
}

void Encrypt::view() {
    auto *btn = (new qontrol::Row)
                    ->pushSpacer()
                    ->push(m_encrypt_btn)
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

auto Encrypt::encryptBtn() -> QPushButton * {
    return m_encrypt_btn;
}

auto Encrypt::widget() -> QWidget * {
    return m_widget;
}

void Encrypt::update(RustScreen screen) {
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
    m_encrypted = !screen.ciphertext.empty();

    // update all keys
    for (size_t i = 0; i < screen.keys.size(); i++) {
        auto txt = QString(screen.keys.at(i).c_str());
        if (txt != m_keys.at(i)->text()) {
            m_keys.at(i)->setText(txt, screen.valid.at(i));
        }
        m_keys.at(i)->setChecked(screen.selected.at(i));
        m_keys.at(i)->setEnabled(!m_encrypted);
        auto selected = screen.selected.at(i);
        m_keys.at(i)->setChecked(selected);
    }

    // update data
    QString txt;
    if (!m_encrypted) {
        m_data_header->setText("Descriptor:");
        txt = QString(screen.descriptor.c_str());
        m_encrypt_btn->setVisible(true);
        m_save_btn->setVisible(false);
        m_data->setEnabled(true);
    } else {
        m_encrypt_btn->setVisible(false);
        m_save_btn->setVisible(true);
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

    bool selectedKey = false;
    for (auto key : screen.selected) {
        if (key) {
            selectedKey = true;
            break;
        }
    }

    if (screen.descriptor.empty() || screen.keys.empty() || !selectedKey) {
        m_encrypt_btn->setEnabled(false);
    } else {
        m_encrypt_btn->setEnabled(true);
    }

    m_data->setText(txt);

    view();
}

void Encrypt::onFileSelected(const QString &path) {
    AppController::get()->encryptSave(path);
}

void Encrypt::onSelectFile() {
    auto *dialog = new QFileDialog(this);
    dialog->setAcceptMode(QFileDialog::AcceptMode::AcceptSave);
    dialog->selectFile("bed.descriptor");
    connect(dialog, &QFileDialog::fileSelected, this, &Encrypt::onFileSelected,
            qontrol::UNIQUE);
    AppController::execModal(dialog);
}

} // namespace screen
