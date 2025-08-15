#pragma once

#include <QEvent>
#include <QHash>
#include <QTabWidget>
#include <QWidget>
#include <Qontrol>
#include <QtLogging>

class MainWindow : public qontrol::Window {
    Q_OBJECT

public:
    explicit MainWindow(QWidget *parent = nullptr);
    ~MainWindow() override;

    void insertTab(QWidget *widget, const QString &name);
    void removeTab(const QString &name);
    void updateTabs();

    auto tabWidget() -> QTabWidget *;
    auto tabs() -> QHash<QString, QWidget *> &;

    // handle drag n drop
    void dragEnterEvent(QDragEnterEvent *event) override;
    void dropEvent(QDropEvent *event) override;

signals:
    void dropped(QString file_path);

protected:
    void closeEvent(QCloseEvent *event) override;

private:
    bool m_init = false;
    QTabWidget *m_tab = nullptr;
    QList<QPair<QString, QWidget *>> m_tabs;

    void initWindow();
};
