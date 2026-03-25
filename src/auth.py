from flask import Blueprint, render_template, redirect, url_for, request
from flask_login import login_user, login_required, logout_user, current_user
from werkzeug.security import generate_password_hash

from . import db
from .models import *


auth = Blueprint("auth", __name__)


@auth.route("/login", methods=['GET', 'POST'])
def login():
    form = LogIn()

    # POST behavior
    if form.validate_on_submit():
        email = form.email.data
        remember = form.remember.data

        user = User.query.filter_by(email=email).first()
        login_user(user, remember=remember)
        return redirect(url_for('main.index'))

    # GET behavior
    return render_template(
        "auth.html",
        form=form,
        title=_("Log In"),
        action=url_for('auth.login'),
        methods=['GET', 'POST']
    )
