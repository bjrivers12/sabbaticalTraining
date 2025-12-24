import tkinter as tk
from tkinter import ttk


def greet():
    print("Hello, World!")

root = tk.Tk()

greet_button = ttk.Button(root, text="Greet", command=greet)
#expand true grows in the window fill makes it take up the reserved space and fill both takes up all space doesn't pprevent others from taking space
greet_button.pack(side="left", fill="y", expand=True)
quit_button = ttk.Button(root, text="Quit", command=root.destroy)
quit_button.pack(side="left")

root.mainloop()

