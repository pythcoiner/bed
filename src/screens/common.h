#pragma once

#include <QFrame>
#include <QScrollArea>
#include <QWidget>

const int MARGIN = 30;
const int LEFT_MARGIN = MARGIN;
const int TOP_MARGIN = MARGIN;
const int RIGHT_MARGIN = MARGIN;
const int BOTTOM_MARGIN = MARGIN;

const int LABEL_WIDTH = 120;
const int INPUT_WIDTH = 200;
const int PRICE_WIDTH = 200;

const int V_SPACER = 5;
const int H_SPACER = 5;

const int BTN_WIDTH = 100;

auto margin(QWidget *widget) -> QWidget *;
auto margin(QWidget *widget, int margin) -> QWidget *;
auto frame(QWidget *widget) -> QWidget *;
auto scrollable_v(QWidget *widget) -> QWidget *;

class Frame : public QFrame {
    Q_OBJECT
public:
    Frame(QWidget *parent = nullptr) : QFrame(parent) {
        setAttribute(
            Qt::WA_TranslucentBackground); // optional if using transparency
    }

protected:
    void paintEvent(QPaintEvent *event) override;
};
