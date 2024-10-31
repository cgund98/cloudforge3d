from . import deps

bl_info = {
    "name": "CloudForge3D",
    "blender": (4, 2, 0),
    "category": "Render",
}

def register():
    deps.register()


def unregister():
    deps.unregister()