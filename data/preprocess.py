from sqlalchemy.orm import (
    declarative_base,
    mapped_column,
    Session,
    Mapped,
)
from sqlalchemy import create_engine
from datetime import date
import pandas as pd
import pathlib

# Download the kaggle dataset.
# https://www.kaggle.com/datasets/utkarshx27/movies-dataset.

Base = declarative_base()


class Movies(Base):
    __tablename__ = "movies"

    id: Mapped[int] = mapped_column(primary_key=True)
    budget: Mapped[int] = mapped_column()
    genres: Mapped[str] = mapped_column()
    keywords: Mapped[str] = mapped_column()
    overview: Mapped[str] = mapped_column()
    popularity: Mapped[float] = mapped_column()
    release_date: Mapped[date] = mapped_column()
    revenue: Mapped[int] = mapped_column()
    title: Mapped[str] = mapped_column()
    vote_average: Mapped[float] = mapped_column()
    vote_count: Mapped[int] = mapped_column()


data = pathlib.Path(__file__).parent / "movie_dataset.csv"

columns = [
    "budget",
    "genres",
    "id",
    "keywords",
    "overview",
    "popularity",
    "release_date",
    "revenue",
    "title",
    "vote_average",
    "vote_count",
]

df = pd.read_csv(filepath_or_buffer=data, parse_dates=["release_date"])
df.dropna(axis=0, inplace=True, how="any")
df = df.loc[:, columns]

datapoints = [row.to_dict() for _, row in df.iterrows()]

# Engine = create_engine("sqlite:///:memory:", echo=False)
Engine = create_engine("sqlite:///database.db", echo=False)
Base.metadata.create_all(Engine)

with Session(Engine) as sess:
    sess.add_all([Movies(**datapoint) for datapoint in datapoints])
    sess.commit()

    # sanity check.
    num_rows = sess.query(Movies).count()
    assert num_rows == len(datapoints)

    # sanity check.
    ids = [d[0] for d in sess.query(Movies.id).all()]
    assert sorted(ids) == sorted([int(d["id"]) for d in datapoints])

# con = Engine.raw_connection()

# with open("dump.sql", "w") as f:
#     for line in con.iterdump():
#         f.write('%s\n' % line)