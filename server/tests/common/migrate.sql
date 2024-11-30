CREATE TABLE movies (
    id INTEGER PRIMARY KEY,
    budget INTEGER NOT NULL,
    revenue INTEGER NOT NULL,
    vote_count INTEGER NOT NULL,
    popularity REAL NOT NULL,
    vote_average REAL NOT NULL,
    genres TEXT NOT NULL,
    title TEXT NOT NULL,
    keywords TEXT NOT NULL,
    overview TEXT NOT NULL,
    release_date TEXT NOT NULL
);

INSERT INTO
    movies (
        budget,
        revenue,
        vote_count,
        popularity,
        vote_average,
        genres,
        title,
        keywords,
        overview,
        release_date
    )
VALUES
    (
        5500000,
        677945399,
        14075,
        21.946943,
        7.7,
        'Adventure, Fantasy, Action',
        'Harry Potter and the Order of the Phoenix',
        'saving the world, witch, magic, sorcery',
        'Harry''s fifth year of study at Hogwarts.',
        '2007-06-28'
    ),
    (
        6000000,
        100853753,
        14075,
        21.946943,
        7.7,
        'Adventure, Fantasy, Action',
        'Harry Potter and the Half-Blood Prince',
        'witch, magic, broom, school of witchcraft',
        'As Harry begins his sixth year at Hogwarts, he discovers an old book marked as "the property of the Half-Blood Prince" and begins to learn more about Lord Voldemort''s dark past.',
        '2009-07-07'
    ),
    (
        7000000,
        961000000,
        14075,
        21.946943,
        7.7,
        'Adventure, Fantasy, Action',
        'Harry Potter and the Deathly Hallows: Part 1',
        'corruption, isolation, radio, teleportation',
        'Harry, Ron, and Hermione search for Voldemort''s remaining Horcruxes in their effort to destroy the Dark Lord as the final battle rages on at Hogwarts.',
        '2010-11-19'
    ),
    (
        8000000,
        1341511219,
        14075,
        21.946943,
        7.7,
        'Adventure, Fantasy, Action',
        'Harry Potter and the Deathly Hallows: Part 2',
        'witch, magic, broom, school of witchcraft',
        'Harry, Ron, and Hermione search for Voldemort''s remaining Horcruxes in their effort to destroy the Dark Lord as the final battle rages on at Hogwarts.',
        '2011-07-15'
    ),
    (
        9000000,
        100853753,
        14075,
        21.946943,
        7.7,
        'Adventure, Fantasy, Action',
        'Harry Potter and the Goblet of Fire',
        'witch, magic, broom, school of witchcraft',
        'Harry finds himself mysteriously selected as an under-aged competitor in a dangerous tournament between three schools of magic.',
        '2005-11-18'
    ),
    (
        10000000,
        100853753,
        14075,
        21.946943,
        7.7,
        'Adventure, Fantasy, Action',
        'Harry Potter and the Prisoner of Azkaban',
        'witch, magic, broom school of witchcraft',
        'It''s Harry''s third year at Hogwarts; not only does he have a new "Defense Against the Dark Arts" teacher, but there is also trouble brewing.',
        '2004-06-04'
    ),
    (
        11000000,
        100853753,
        14075,
        21.946943,
        7.7,
        'Adventure, Fantasy, Action',
        'Harry Potter and the Chamber of Secrets',
        'witch, magic, broom, school of witchcraft',
        'An ancient prophecy seems to be coming true when a mysterious presence begins stalking the corridors of a school of magic and leaving its victims paralyzed.',
        '2002-11-15'
    );