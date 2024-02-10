#include <QLabel>
#include <QMouseEvent>
#include <QtWidgets>

int main(int argc, char* argv[]) {
  QApplication app(argc, argv);
  QWidget window;
  QLabel* label = new QLabel(&window);
  window.setWindowTitle("First window");
  window.resize(960, 540);
  // window.showMaximized();
  label->setAlignment(Qt::AlignLeft);
  label->setText("Some text: so far");
  label->adjustSize();
  window.show();
  return app.exec();
}
