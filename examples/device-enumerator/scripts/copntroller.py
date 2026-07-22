import vgamepad as vg
import time

gamepad = vg.VX360Gamepad()

print("Gamepad virtuale creato.")
print("Premi CTRL+C per chiudere.")

try:
    while True:
        gamepad.update()
        time.sleep(0.01)

except KeyboardInterrupt:
    print("Chiusura...")