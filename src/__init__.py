from flask import Flask, request, render_template, url_for
from flask_login import LoginManager, current_user
from flask_sqlalchemy import SQLAlchemy
import json
import os


db = SQLAlchemy()
ADMIN_USER_IDS = 1,


def create_app():
    """Creates and configures a Flask object. Called at the start of any new session.

    Returns:
        A Falsk object
    """

    app = Flask(__name__)

    # Configure from environment variables
    app.config['SECRET_KEY'] = os.environ.get("SECRET_KEY")
    app.config['SQLALCHEMY_DATABASE_URI'] = os.environ.get("DATABASE_URL", '')\
        .replace("postgres://", "postgresql://") or "sqlite:///db.sqlite"

    # Configure from a config file
    try:
        app.config.from_file("config.json", load=json.load)
    except:
        pass

    db.init_app(app)

    login_manager = LoginManager()
    login_manager.login_view = 'auth.login'
    login_manager.init_app(app)
    from .models import User

    @login_manager.user_loader
    def load_user(user_id):
        return User.query.get(int(user_id))

    with app.app_context():
        db.create_all()

    from .auth import auth as auth_blueprint

    return app
