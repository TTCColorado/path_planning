from path_planning import SpaceConf, RobotConf, RRTDubinsPlanner, plan_async
import math
import asyncio

rc = RobotConf(1.0, 1.0, 0.8)
sc = SpaceConf([(-6.0, -6.0), (-6.0, 15.0), (15.0, 15.0), (15.0, -6.0), (-6.0, -6.0)], [])
sy = math.radians(-45.0)
gy = math.radians(45.0)
p = RRTDubinsPlanner((-5.0, -5.0), sy, (6.0, 10.0), gy, 5000, 0.1, sc, rc)

asyncio.run(plan_async(p))
