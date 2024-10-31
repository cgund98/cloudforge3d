import bpy


class Properties(bpy.types.PropertyGroup):

    memory_gb: bpy.props.EnumProperty(
        name="Memory (GB)",
        description="Memory (GB)",
        items=[
            ("15", "15GB", ""),
            ("29", "29GB", ""),
            ("47", "47GB", ""),
            ("59", "59GB", "")
        ],
        default="15",
    )

    render_mode: bpy.props.EnumProperty(
        name="Render Mode",
        description="Render Mode",
        items=[
            ("ANIM", "Sequence", ""),
            ("FRAME", "Frame", ""),
        ],
        default="FRAME",
    )
