#include <MainWindow.h>
#include <QApplication>
#include <Qontrol>
#include <qapplication.h>

auto main(int argc, char *argv[]) -> int {
    QApplication app(argc, argv);

    QFont font("Noto Sans CJK", 12);
    QApplication::setFont(font);

    MainWindow window;
    window.setFixedSize(600, 500);
    window.setWindowTitle("Bitcoin Encrypted Descriptor");
    window.show();

    return QApplication::exec();
}
