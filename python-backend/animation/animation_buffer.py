class AnimationBuffer:

    def __init__(self):
        self.log = open('animation.log', 'a')
        self.log.write("python_001\n")

    def write(self, data_structure, operation, id, data) -> None:
        self.log.write(f"{data_structure}_{id}:{operation}({data})\n")

    