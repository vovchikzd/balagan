#include <QLabel>
#include <QMouseEvent>
#include <QtWidgets>

int main(int argc, char* argv[]) {
  QApplication app(argc, argv);
  QWidget window;
  QLabel* label = new QLabel(&window);
  window.setWindowTitle("First window");
  window.resize(960, 540);
  window.showMaximized();
  QString str =
      "This is have to long text, so I trying to make it more ane more "
      "bigger.\nEven more, and again. Probably and again.\nAng over, and over "
      "again.";
  label->setAlignment(Qt::AlignLeft);
  label->setText(str);
  label->adjustSize();
  window.show();
  return app.exec();
}
