import tkinter as tk
from tkinter import ttk


def greet():
    print(f"Hello, {user_name.get() or 'World'}")

root = tk.Tk()

user_name = tk.StringVar()

name_label = ttk.Label(root, text="Name: ")
name_label.pack(side="left", padx=(0,10))
name_entry = ttk.Entry(root, width=15, textvariable=user_name)
name_entry.pack(side="left")
name_entry.focus()


greet_button = ttk.Button(root, text="Greet", command=greet)
#expand true grows in the window fill makes it take up the reserved space and fill both takes up all space doesn't pprevent others from taking space
greet_button.pack(side="left", fill="y", expand=True)
quit_button = ttk.Button(root, text="Quit", command=root.destroy)
quit_button.pack(side="left")

root.mainloop()

