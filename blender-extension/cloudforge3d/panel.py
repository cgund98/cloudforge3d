
import bpy

from cloudforge3d.operator import CreateJobOperator

class RootPanel(bpy.types.Panel):
    bl_space_type = "PROPERTIES"
    bl_region_type = "WINDOW"
    bl_context = "render"

    bl_idname = "cf3d.panel.root"
    bl_label = "CloudForge3D"

    def draw(self, context: bpy.types.Context):
        """Render UI components"""
        props = context.scene.cf3d

        col = self.layout.column(align=True)
        
        # Header
        box = col.box()
        box.scale_y = 1.3
        row = box.row()
        row.label(icon="PREFERENCES")
        row.label(text="Job Submission")

         # Body
        box = col.box()
        box.scale_y = 1.3

        row = box.row(align=True)
        row.prop(props, "memory_gb")

        row = box.row(align=True)
        row.prop(props, "render_mode")

        row = self.layout.row()
        row.operator(CreateJobOperator.bl_idname, text="Submit Job")

