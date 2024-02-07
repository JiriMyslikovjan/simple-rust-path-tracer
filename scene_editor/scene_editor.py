import tkinter as tk
from tkinter import messagebox, filedialog
import json
import re
import os

MATERIAL_TYPES = ["diffuse", "specular", "refractive"]

class SceneEditor:
    def __init__(self, master):
        self.master = master
        self.master.title("Scene Editor")
        
        self.objects = []
        
        # Add Sphere button
        add_sphere_btn = tk.Button(self.master, text="Add Sphere", command=self.add_sphere_window)
        add_sphere_btn.grid(row=0, column=0)
        
        # Add Plane button
        add_plane_btn = tk.Button(self.master, text="Add Plane", command=self.add_plane_window)
        add_plane_btn.grid(row=0, column=1)
        
        # Add Light button
        add_light_btn = tk.Button(self.master, text="Add Light", command=self.add_light_window)
        add_light_btn.grid(row=0, column=2)
        
        # Add Camera button
        add_camera_btn = tk.Button(self.master, text="Add Camera", command=self.add_camera_window)
        add_camera_btn.grid(row=0, column=3)
        
        # Save Scene button
        save_scene_btn = tk.Button(self.master, text="Save Scene", command=self.save_scene)
        save_scene_btn.grid(row=0, column=4)
        
        # Scene Listbox
        self.scene_listbox = tk.Listbox(self.master, width=140, height=20)
        self.scene_listbox.grid(row=1, columnspan=5)
        
    def add_sphere_window(self):
        sphere_window = tk.Toplevel(self.master)
        sphere_window.title("Add Sphere")
        
        label = tk.Label(sphere_window, text="Sphere Parameters")
        label.grid(row=0, columnspan=2)
        
        center_label = tk.Label(sphere_window, text="Center:")
        center_label.grid(row=1, column=0)
        center_x_entry = tk.Entry(sphere_window)
        center_x_entry.grid(row=1, column=1)
        center_y_entry = tk.Entry(sphere_window)
        center_y_entry.grid(row=1, column=2)
        center_z_entry = tk.Entry(sphere_window)
        center_z_entry.grid(row=1, column=3)
        
        radius_label = tk.Label(sphere_window, text="Radius:")
        radius_label.grid(row=2, column=0)
        radius_entry = tk.Entry(sphere_window)
        radius_entry.grid(row=2, column=1)
        
        material_label = tk.Label(sphere_window, text="Material Type:")
        material_label.grid(row=4, column=0)
        material_var = tk.StringVar(sphere_window)
        material_var.set(MATERIAL_TYPES[0])
        material_dropdown = tk.OptionMenu(sphere_window, material_var, *MATERIAL_TYPES)
        material_dropdown.grid(row=4, column=1)
        
        color_label = tk.Label(sphere_window, text="Color (#RRGGBB):")
        color_label.grid(row=5, column=0)
        color_entry = tk.Entry(sphere_window)
        color_entry.grid(row=5, column=1)
        
        add_btn = tk.Button(sphere_window, text="Add", command=lambda: self.add_sphere(
            center_x_entry.get(), center_y_entry.get(), center_z_entry.get(),
            radius_entry.get(), material_var.get(), color_entry.get()))
        add_btn.grid(row=7, columnspan=2)

        add_btn.configure(command=lambda: [self.add_sphere(center_x_entry.get(), center_y_entry.get(),
                                                      center_z_entry.get(), radius_entry.get(),
                                                      material_var.get(), color_entry.get()),
                                       sphere_window.destroy()])
        
    def add_sphere(self, center_x, center_y, center_z, radius, material_type, color):
        try:
            center = [float(center_x), float(center_y), float(center_z)]
            radius = float(radius)

            if not re.match(r"^#[0-9a-fA-F]{6}$", color):
                raise ValueError
            
            color = color.upper()

            sphere = {
                "type": "sphere",
                "center": center,
                "radius": radius,
                "material": {
                    "type": material_type,
                    "color": color
                }
            }
            
            self.objects.append(sphere)
            self.update_scene_list()
            
        except ValueError:
            messagebox.showerror("Error", "Invalid input")
            
    def add_plane_window(self):
        plane_window = tk.Toplevel(self.master)
        plane_window.title("Add Plane")
        
        label = tk.Label(plane_window, text="Plane Parameters")
        label.grid(row=0, columnspan=2)
        
        normal_label = tk.Label(plane_window, text="Normal:")
        normal_label.grid(row=1, column=0)
        normal_x_entry = tk.Entry(plane_window)
        normal_x_entry.grid(row=1, column=1)
        normal_y_entry = tk.Entry(plane_window)
        normal_y_entry.grid(row=1, column=2)
        normal_z_entry = tk.Entry(plane_window)
        normal_z_entry.grid(row=1, column=3)
        
        d_label = tk.Label(plane_window, text="d:")
        d_label.grid(row=2, column=0)
        d_entry = tk.Entry(plane_window)
        d_entry.grid(row=2, column=1)
        
        material_label = tk.Label(plane_window, text="Material Type:")
        material_label.grid(row=3, column=0)
        material_var = tk.StringVar(plane_window)
        material_var.set(MATERIAL_TYPES[0])
        material_dropdown = tk.OptionMenu(plane_window, material_var, *MATERIAL_TYPES)
        material_dropdown.grid(row=3, column=1)
        
        color_label = tk.Label(plane_window, text="Color (#RRGGBB):")
        color_label.grid(row=4, column=0)
        color_entry = tk.Entry(plane_window)
        color_entry.grid(row=4, column=1)
        
        add_btn = tk.Button(plane_window, text="Add", command=lambda: self.add_plane(
            normal_x_entry.get(), normal_y_entry.get(), normal_z_entry.get(),
            d_entry.get(), material_var.get(), color_entry.get()))
        
        add_btn.grid(row=7, columnspan=2)

        add_btn.configure(command=lambda: [self.add_plane(normal_x_entry.get(), normal_y_entry.get(),
                                                      normal_z_entry.get(), d_entry.get(),
                                                      material_var.get(), color_entry.get()),
                                       plane_window.destroy()])
        
    def add_plane(self, normal_x, normal_y, normal_z, d, material_type, color):
        try:
            normal = [float(normal_x), float(normal_y), float(normal_z)]
            d = float(d)
            
            if not re.match(r"^#[0-9a-fA-F]{6}$", color):
                raise ValueError
            
            color = color.upper()
            
            plane = {
                "type": "plane",
                "normal": normal,
                "d": d,
                "material": {
                    "type": material_type,
                    "color": color
                }
            }
            
            self.objects.append(plane)
            self.update_scene_list()
            
        except ValueError:
            messagebox.showerror("Error", "Invalid input")
    
    def add_light_window(self):
        light_window = tk.Toplevel(self.master)
        light_window.title("Add Light")

        label = tk.Label(light_window, text="Light Parameters")
        label.grid(row=0, columnspan=2)

        center_label = tk.Label(light_window, text="Center:")
        center_label.grid(row=1, column=0)
        center_x_entry = tk.Entry(light_window)
        center_x_entry.grid(row=1, column=1)
        center_y_entry = tk.Entry(light_window)
        center_y_entry.grid(row=1, column=2)
        center_z_entry = tk.Entry(light_window)
        center_z_entry.grid(row=1, column=3)

        radius_label = tk.Label(light_window, text="Radius:")
        radius_label.grid(row=2, column=0)
        radius_entry = tk.Entry(light_window)
        radius_entry.grid(row=2, column=1)

        emission_label = tk.Label(light_window, text="Emission:")
        emission_label.grid(row=3, column=0)
        emission_entry = tk.Entry(light_window)
        emission_entry.grid(row=3, column=1)

        add_btn = tk.Button(light_window, text="Add", command=lambda: self.add_light(
            center_x_entry.get(), center_y_entry.get(), center_z_entry.get(),
            radius_entry.get(), emission_entry.get()))
        add_btn.grid(row=6, columnspan=2)

        add_btn.configure(command=lambda: [self.add_light(center_x_entry.get(), center_y_entry.get(),
                                                        center_z_entry.get(), radius_entry.get(),
                                                        emission_entry.get()),
                                        light_window.destroy()])

        
    def add_light(self, center_x, center_y, center_z, radius, emission):
        try:
            center = [float(center_x), float(center_y), float(center_z)]
            radius = float(radius)
            emission = float(emission)

            light = {
            "type": "light",
            "center": center,
            "radius": radius,
            "emission": emission
            }
        
            
            self.objects.append(light)
            self.update_scene_list()
            
        except ValueError:
            messagebox.showerror("Error", "Invalid input")
    
    def add_camera_window(self):
        camera_window = tk.Toplevel(self.master)
        camera_window.title("Add Camera")
        
        label = tk.Label(camera_window, text="Camera Parameters")
        label.grid(row=0, columnspan=2)
        
        look_from_label = tk.Label(camera_window, text="Look From:")
        look_from_label.grid(row=1, column=0)
        look_from_x_entry = tk.Entry(camera_window)
        look_from_x_entry.grid(row=1, column=1)
        look_from_y_entry = tk.Entry(camera_window)
        look_from_y_entry.grid(row=1, column=2)
        look_from_z_entry = tk.Entry(camera_window)
        look_from_z_entry.grid(row=1, column=3)
        
        look_at_label = tk.Label(camera_window, text="Look At:")
        look_at_label.grid(row=2, column=0)
        look_at_x_entry = tk.Entry(camera_window)
        look_at_x_entry.grid(row=2, column=1)
        look_at_y_entry = tk.Entry(camera_window)
        look_at_y_entry.grid(row=2, column=2)
        look_at_z_entry = tk.Entry(camera_window)
        look_at_z_entry.grid(row=2, column=3)
        
        vup_label = tk.Label(camera_window, text="Vup:")
        vup_label.grid(row=3, column=0)
        vup_x_entry = tk.Entry(camera_window)
        vup_x_entry.grid(row=3, column=1)
        vup_y_entry = tk.Entry(camera_window)
        vup_y_entry.grid(row=3, column=2)
        vup_z_entry = tk.Entry(camera_window)
        vup_z_entry.grid(row=3, column=3)
        
        fov_label = tk.Label(camera_window, text="Vertical FOV:")
        fov_label.grid(row=4, column=0)
        fov_entry = tk.Entry(camera_window)
        fov_entry.grid(row=4, column=1)
        
        resolution_label = tk.Label(camera_window, text="Resolution (width x height):")
        resolution_label.grid(row=5, column=0)
        width_entry = tk.Entry(camera_window)
        width_entry.grid(row=5, column=1)
        height_entry = tk.Entry(camera_window)
        height_entry.grid(row=5, column=2)
        
        add_btn = tk.Button(camera_window, text="Add", command=lambda: self.add_camera(
            look_from_x_entry.get(), look_from_y_entry.get(), look_from_z_entry.get(),
            look_at_x_entry.get(), look_at_y_entry.get(), look_at_z_entry.get(),
            vup_x_entry.get(), vup_y_entry.get(), vup_z_entry.get(),
            fov_entry.get(), width_entry.get(), height_entry.get()))
        add_btn.grid(row=13, columnspan=2)

        add_btn.configure(command=lambda: [self.add_camera(look_from_x_entry.get(), look_from_y_entry.get(),
                                                       look_from_z_entry.get(), look_at_x_entry.get(),
                                                       look_at_y_entry.get(), look_at_z_entry.get(),
                                                       vup_x_entry.get(), vup_y_entry.get(), vup_z_entry.get(),
                                                       fov_entry.get(), width_entry.get(), height_entry.get()),
                                       camera_window.destroy()])
        
    def add_camera(self, look_from_x, look_from_y, look_from_z, look_at_x, look_at_y, look_at_z,
                   vup_x, vup_y, vup_z, fov, width, height):
        try:
            look_from = [float(look_from_x), float(look_from_y), float(look_from_z)]
            look_at = [float(look_at_x), float(look_at_y), float(look_at_z)]
            vup = [float(vup_x), float(vup_y), float(vup_z)]
            fov = float(fov)
            width = int(width)
            height = int(height)

            camera_count = sum(obj["type"] == "camera" for obj in self.objects)
            
            if camera_count >= 1:
                messagebox.showerror("Error", "The scene can have only 1 camera.")
                return
            
            camera = {
                "type": "camera",
                "look_from": look_from,
                "look_at": look_at,
                "vup": vup,
                "fov": fov,
                "resolution": {
                    "width": width,
                    "height": height
                }
            }
            
            self.objects.append(camera)
            self.update_scene_list()
            
        except ValueError:
            messagebox.showerror("Error", "Invalid input")
    
    def update_scene_list(self):
        self.scene_listbox.delete(0, tk.END)
        for i, obj in enumerate(self.objects):
            self.scene_listbox.insert(tk.END, f"Object {i+1}: {obj}")
    
    def save_scene(self):
        if len(self.objects) == 0:
            messagebox.showwarning("Warning", "No objects in the scene")
            return

        camera_count = sum(obj["type"] == "camera" for obj in self.objects)

        if camera_count != 1:
            messagebox.showerror("Error", "The scene must have a camera before saving.")
            return

        # Get the absolute path of the scenes directory
        scenes_path = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "scenes"))

        # Custom file dialog
        root = tk.Tk()
        root.withdraw()
        root.attributes("-topmost", True)
        filename = filedialog.asksaveasfilename(
            initialfile="scene.json",
            defaultextension=".json",
            filetypes=[("JSON Files", "*.json")],
            initialdir=scenes_path
        )
        root.destroy()

        if filename:
            selected_path = os.path.abspath(os.path.dirname(filename))
            
            if selected_path != scenes_path:
                messagebox.showerror("Error", "Invalid save location. Please select the 'scenes' folder.")
                return

            scene = {
                "objects": self.objects
            }

            with open(filename, "w") as file:
                json.dump(scene, file)

            messagebox.showinfo("Success", "Scene saved successfully")

            self.master.destroy()

root = tk.Tk()
editor = SceneEditor(root)
root.mainloop()
