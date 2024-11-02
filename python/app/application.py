from PyQt6.QtWidgets import QApplication, QMainWindow
import sys

app = QApplication(sys.argv)

window = QMainWindow()
window.resize(400, 300)


window.show()
sys.exit(app.exec())
