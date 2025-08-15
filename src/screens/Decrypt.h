#pragma once

#include "../AppController.h"
#include "include/bed.h"
#include "widgets/Key.h"
#include <QFileDialog>
#include <QGuiApplication>
#include <QLabel>
#include <QLineEdit>
#include <QPushButton>
#include <QTextEdit>
#include <QWidget>
#include <Qontrol>
#include <cstddef>

namespace screen {

class Decrypt : public qontrol::Screen {
    Q_OBJECT
public:
    explicit Decrypt(QWidget *parent);

    ~Decrypt() override;
    auto decryptBtn() -> QPushButton *;
    auto widget() -> QWidget *;

    void update(RustScreen screen);

public slots:

    void onSelectFile();
    void onFileSelected(const QString &path);
    void onCopy();

signals:
    void deleteKey(size_t index);
    void editKey(size_t index, QString txt);
    void selectKey(size_t index, bool selected);
    void addKey();
    void tryDecrypt();

protected:
    void view() override;
    void init() override;
    void doConnect() override;

private:
    QWidget *m_widget = nullptr;
    QLabel *m_key_header = nullptr;
    QList<widget::Key *> m_keys;
    QPushButton *m_add_key = nullptr;
    QLabel *m_data_header = nullptr;
    QTextEdit *m_data = nullptr;
    QPushButton *m_decrypt_btn = nullptr;
    QPushButton *m_save_btn = nullptr;
    QPushButton *m_copy_btn = nullptr;
    QPushButton *m_reset_btn = nullptr;
    bool m_decrypted = false;
    QWidget *m_scroll = nullptr;
};

} // namespace screen
