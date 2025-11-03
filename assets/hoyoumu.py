"""Game fix for Genshin Impact and ZZZ"""

from protonfixes import util


def main() -> None:
    util.set_environment('UMU_USE_STEAM', '1')
    util.set_game_drive(True)
