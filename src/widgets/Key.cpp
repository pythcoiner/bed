#include "Key.h"
#include "../AppController.h"
#include "common.h"
#include <QCheckBox>

namespace widget {

Key::Key(QWidget *parent) {
    m_checkbox = new QCheckBox;
    connect(m_checkbox, &QCheckBox::toggled, this, &Key::setChecked,
            qontrol::UNIQUE);

    m_input = new QLineEdit;
    m_input->setFixedWidth(350);
    m_valid = new QLabel;
    m_valid->setFixedWidth(30);

    auto *input = m_input;
    auto *vLabel = m_valid;

    connect(m_input, &QLineEdit::textChanged, [input, vLabel, this]() {
        auto valid = AppController::isKeyValid(input->text());
        if (!input->text().isEmpty()) {
            setValid(valid);

        } else {
            setEmpty();
        }
    });
    m_delete = new QPushButton;
    auto cross = m_delete->style()->standardIcon(
        QStyle::SP_TitleBarCloseButton);
    m_delete->setIcon(cross);
    this->push(m_checkbox)
        ->pushSpacer(H_SPACER)
        ->push(m_input)
        ->pushSpacer(H_SPACER)
        ->push(m_valid)
        ->pushSpacer(H_SPACER)
        ->push(m_delete)
        ->pushSpacer();
    this->setParent(parent);
}

Key::~Key() {
}

auto Key::text() -> QString {
    return m_input->text();
};

void Key::setChecked(bool checked) {
    m_checkbox->setChecked(checked);
    m_input->setEnabled(!checked);
}

void Key::setText(const QString &txt, bool valid) {
    m_input->setText(txt);
    if (!txt.isEmpty()) {
        setValid(valid);
    } else {
        m_valid->clear();
    }
}

void Key::setValid(bool valid) {
    if (valid) {
        m_valid->setText(QString::fromUtf8("\u2705"));
        m_checkbox->setEnabled(true);
    } else {
        m_valid->setText(QString::fromUtf8("\u26A0"));
        m_checkbox->setEnabled(false);
        m_checkbox->setChecked(false);
    }
}

auto Key::delBtn() -> QPushButton * {
    return m_delete;
}

auto Key::input() -> QLineEdit * {
    return m_input;
}

auto Key::checkbox() -> QCheckBox * {
    return m_checkbox;
}
} // namespace widget
