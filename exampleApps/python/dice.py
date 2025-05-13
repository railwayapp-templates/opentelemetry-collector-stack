import random
from instrumentation import tracer


def roll_once(i: int, min_val: int, max_val: int) -> int:
    """
    Roll a single die with tracing.

    Args:
        i (int): Roll index for span naming
        min_val (int): Minimum value (inclusive)
        max_val (int): Maximum value (exclusive)

    Returns:
        int: Result of the die roll
    """
    with tracer.start_as_current_span(f"rollOnce:{i}") as span:
        result = random.randint(min_val, max_val - 1)
        span.set_attribute("dicelib.rolled", str(result))
        return result


def roll_dice(num_dice: int) -> list:
    """
    Roll multiple dice with tracing.

    Args:
        num_dice (int): Number of dice to roll (1-100)

    Returns:
        list: Results of all dice rolls

    Raises:
        ValueError: If num_dice is not between 1 and 100
    """
    sides = 6  # Always 6-sided dice as per app.py logic
    if not (1 <= num_dice <= 100):
        raise ValueError("Number of dice must be between 1 and 100.")
    with tracer.start_as_current_span("rollTheDice") as span:
        results = [roll_once(i, 1, sides + 1) for i in range(num_dice)]
        span.add_event("Dice rolled", {"results": results, "total": sum(results)})
        return results
