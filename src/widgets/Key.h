#pragma once

#include "../screens/common.h"
#include "Row.h"
#include <QCheckBox>
#include <QLabel>
#include <QLineEdit>
#include <QObject>
#include <QPushButton>
#include <QStyle>
#include <QWidget>

namespace widget {

class Key : public qontrol::Row {
    Q_OBJECT
public:
    Key(QWidget *parent = nullptr);

    ~Key() override;

    auto text() -> QString;
    auto delBtn() -> QPushButton *;
    auto input() -> QLineEdit *;
    auto checkbox() -> QCheckBox *;

public slots:
    void setText(const QString &txt, bool valid);
    void setChecked(bool checked);
    void setValid(bool valid);

    void setEmpty() {
        m_valid->setText(QString::fromUtf8(""));
    }

private:
    QCheckBox *m_checkbox = nullptr;
    QLineEdit *m_input = nullptr;
    QLabel *m_valid = nullptr;
    QPushButton *m_delete = nullptr;
};

} // namespace widget
