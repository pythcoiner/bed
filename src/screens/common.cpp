#include "common.h"
#include "Column.h"
#include "Row.h"
#include <QBoxLayout>
#include <QFrame>
#include <QPainter>
#include <QPen>

auto margin(QWidget *widget) -> QWidget * {
    return margin(widget, MARGIN);
}

auto margin(QWidget *widget, int margin) -> QWidget * {
    auto *col = (new qontrol::Column)
                    ->pushSpacer(margin)
                    ->push(widget)
                    ->pushSpacer(margin);
    auto *row = (new qontrol::Row)
                    ->pushSpacer(margin)
                    ->push(col)
                    ->pushSpacer(margin);
    return row;
}

auto frame(QWidget *widget) -> QWidget * {
    auto *frame = new Frame;
    frame->setFrameShape(QFrame::Box);     // Or Panel, StyledPanel, etc.
    frame->setFrameShadow(QFrame::Sunken); // Or Raised, Sunken
    auto *layout = new QVBoxLayout(frame);
    layout->addWidget(widget);
    widget->setParent(frame);
    int m = 10;
    layout->setContentsMargins(m, m, m, m);
    return frame;
}

void Frame::paintEvent(QPaintEvent *event) {
    QPainter painter(this);
    painter.setRenderHint(QPainter::Antialiasing, true);

    QPen pen(Qt::darkGray, 3);
    painter.setPen(pen);

    QRectF rect = this->rect();
    painter.drawRoundedRect(rect, 10, 10);
}

auto scrollable_v(QWidget *widget) -> QWidget * {
    auto *scroll = new QScrollArea;
    scroll->setWidget(widget);
    scroll->setHorizontalScrollBarPolicy(
        Qt::ScrollBarPolicy::ScrollBarAlwaysOff);
    return scroll;
}
