from flask_login import UserMixin

from . import db


class User(UserMixin, db.Model):

    id = db.Column(db.Integer, primary_key=True)
    created = db.Column(db.DateTime)

    email = db.Column(db.String(100), unique=True)
    password = db.Column(db.String(500))
    firstname = db.Column(db.String(100))
    lastname = db.Column(db.String(100))
    timezone = db.Column(db.String(100), default="Canada/Eastern")
