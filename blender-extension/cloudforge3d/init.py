import bpy

from .panel import RootPanel
from .props import Properties
from .operator import CreateJobOperator

classes = [
    CreateJobOperator,
    Properties,
    RootPanel,
]

def register():
    for cls in classes:
        bpy.utils.register_class(cls)

    bpy.types.Scene.cf3d = bpy.props.PointerProperty(type=Properties)


def unregister():
    del bpy.types.Scene.cf3d

    for cls in classes:
        bpy.utils.unregister_class(cls)