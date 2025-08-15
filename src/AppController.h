#pragma once

#include "../lib/include/bed.h"
#include <QObject>
#include <QSystemTrayIcon>
#include <Qontrol>
#include <cstddef>

namespace screen {
class Decrypt;
} // namespace screen

class AppController : public qontrol::Controller {
    Q_OBJECT
public:
    AppController();
    static void init();
    static auto get() -> AppController *;
    static auto isKeyValid(const QString &key) -> bool;
    static auto isDescriptorValid(const QString &key) -> bool;

signals:
    void updateDecrypt(RustScreen);
    void updateEncrypt(RustScreen);
    void error(QString);

public slots:

    void initController();
    void initState();

    void poll();
    void pollNotif();
    void pollError();

    void handleError(const QString &error);

    // drag n drop
    void dropped(const QString &file_path);

    // from Decrypt screen
    void decryptDeleteKey(size_t index);
    void decryptEditKey(size_t index, const QString &txt);
    void decryptAddKey();
    void decryptSelectKey(size_t index, bool selected);
    void tryDecrypt();
    void decryptSave(const QString &path);
    void decryptReset();

    // from Decrypt screen
    void encryptEditKey(size_t index, const QString &txt);
    void encryptAddKey();
    void encryptDeleteKey(size_t index);
    void encryptSelectKey(size_t index, bool selected);
    void encryptReset();
    void encryptSetDescriptor(const QString &txt);
    void tryEncrypt();
    void encryptSave(const QString &path);

    // mode
    void tabChanged(int index);
    void setMode(Mode mode);

    // OS Notifications
    void osMessage(QString title, QString msg, int delay = 10000);
    void osInfo(QString title, QString msg, int delay = 10000);
    void osWarning(QString title, QString msg, int delay = 10000);
    void osCritical(QString title, QString msg, int delay = 10000);

    void stop();

private:
    QSystemTrayIcon *m_tray_icon = nullptr;
    std::optional<rust::Box<RustController>> m_rust_controller = std::nullopt;
    QTimer *m_notif_timer = nullptr;
    Mode m_mode = Mode::Decrypt;
};
