from bottle import Bottle, request, response, run, static_file, template
import json
import os

from dice import roll_dice

app = Bottle()


# Serve static files (CSS)
@app.get("/static/<filename>")
def serve_static(filename):
    return static_file(filename, root="./static")


# Homepage
@app.get("/")
def homepage():
    return template("public/index.html")


@app.get("/rolldice")
def api_roll():
    """
    API endpoint to roll dice.
    :param rolls: Number of dice to roll (default is 1)
    :return: JSON response with the results of the dice rolls
    """
    # Get query parameters
    # Example: /rolldice?rolls=2
    try:
        num_dice = int(request.query.get("rolls", 1))
        result = roll_dice(num_dice)
        response.content_type = "application/json"
        return json.dumps({"dice": num_dice, "results": result})

    except ValueError as ve:
        response.status = 400
        return {"error": str(ve)}
    except Exception:
        response.status = 500
        return {"error": "Internal server error"}


if __name__ == "__main__":
    host = os.getenv("APP_HOST", "localhost")
    port = int(os.getenv("APP_PORT", "8080"))
    run(app, host="localhost", port=8080, debug=True)
