import functools

import numpy as np
from gymnasium.spaces import Discrete, MultiDiscrete
from pettingzoo.test import parallel_api_test, parallel_test
from pettingzoo import ParallelEnv

from pattingzoo.escape_game import EscapeEnv
from pattingzoo.pattingzoo_rs import Env


def run():
    print("testing python implementation:")
    env = EscapeEnv()
    parallel_api_test(env, num_cycles=1_000_000)

    print("testing rust implementation:")
    env = RustEnvWrapper()
    parallel_api_test(env, num_cycles=1_000_000)

    env = RustEnvWrapper()
    obs, infos = env.reset()

    for _ in range(3):
        actions = {
            agent: parallel_test.sample_action(env, obs, agent) for agent in env.agents
        }
        obs, rew, terminated, truncated, info = env.step(actions)
        env.render()
        if any(terminated.values()):
            print("terminated")
            _ = env.reset()
        if any(truncated.values()):
            print("truncated")
            _ = env.reset()


class RustEnvWrapper(Env, ParallelEnv):
    def render(self):
        """Renders the environment."""
        grid = np.zeros((7, 7), dtype=object)
        y, x = self.prisoner_y, self.prisoner_x
        print(x, y)
        grid[self.prisoner_y, self.prisoner_x] = "P"
        grid[self.guard_y, self.guard_x] = "G"
        grid[self.escape_y, self.escape_x] = "E"
        print(f"{grid} \n")

    # Observation space should be defined here.
    # lru_cache allows observation and action spaces to be memoized, reducing clock cycles required to get each agent's space.
    # If your spaces change over time, remove this line (disable caching).
    @functools.lru_cache(maxsize=None)
    def observation_space(self, agent):
        # gymnasium spaces are defined and documented here: https://gymnasium.farama.org/api/spaces/
        return MultiDiscrete([7 * 7 - 1] * 3)

    # Action space should be defined here.
    # If your spaces change over time, remove this line (disable caching).
    @functools.lru_cache(maxsize=None)
    def action_space(self, agent):
        return Discrete(4)


if __name__ == "__main__":
    run()
