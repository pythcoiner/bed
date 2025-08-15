#include "MainWindow.h"
#include "AppController.h"
#include "common.h"
#include "screens/Decrypt.h"
#include "screens/Encrypt.h"
#include <QMargins>
#include <QTabWidget>
#include <QWidget>
#include <QtLogging>

MainWindow::MainWindow(QWidget *parent) : Window(parent) {
    AppController::init();
    auto *ctrl = AppController::get();
    ctrl->initController();
    initWindow();
    ctrl->start(this);
    ctrl->initState();
    connect(this, &MainWindow::dropped, ctrl, &AppController::dropped,
            qontrol::UNIQUE);
}

void MainWindow::initWindow() {
    if (m_init)
        return;

    m_tab = new QTabWidget(this);
    connect(m_tab, &QTabWidget::currentChanged, AppController::get(),
            &AppController::tabChanged, qontrol::UNIQUE);

    auto *decrypt = new screen::Decrypt(this);
    insertTab(decrypt, "Decrypt");

    auto *encrypt = new screen::Encrypt(this);
    insertTab(encrypt, "Encrypt");

    updateTabs();

    // enable drag n drop
    setAcceptDrops(true);

    setCentralWidget(m_tab);
    m_init = true;
}

void MainWindow::insertTab(QWidget *widget, const QString &name) {
    m_tabs.append(QPair(name, widget));
    updateTabs();
}

void MainWindow::removeTab(const QString &name) {
    auto exists = false;
    int index = 0;
    for (int i = 0; i < m_tabs.size(); ++i) {
        if (m_tabs.at(i).first == name) {
            index = i;
            exists = true;
        }
    }

    if (!exists)
        return;
    m_tabs.removeAt(index);
}

void MainWindow::updateTabs() {
    for (const auto &tab : m_tabs) {
        m_tab->addTab(tab.second, tab.first);
    }
}

void MainWindow::closeEvent(QCloseEvent *event) {
    AppController::get()->stop();
    event->accept();
}

MainWindow::~MainWindow() = default;

void MainWindow::dropEvent(QDropEvent *event) {
    auto url = event->mimeData()->urls().first().toLocalFile();
    event->acceptProposedAction();
    emit dropped(url);
}

void MainWindow::dragEnterEvent(QDragEnterEvent *event) {
    // NOTE: this is needed in order to receive the drop event
    if (event->mimeData()->hasUrls()) {
        auto url = event->mimeData()->urls().first();
        if (url.isLocalFile()) {
            event->acceptProposedAction();
        }
    }
}
