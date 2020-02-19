from .path_planning import SpaceConf, RobotConf, RRTDubinsPlanner, create_circle
import concurrent.futures
import asyncio

__all__ = ["SpaceConf", "RobotConf", "RRTDubinsPlanner", "create_circle"]


async def plan_async(planner, poll_interval=0.1):
    future = planner.plan_async()
    while True:
        n = future.check()
        if future.is_done(n):
            return future.finalize(n)
        await asyncio.sleep(poll_interval)
