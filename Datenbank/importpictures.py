import sqlite3
from os import walk


def convertToBinaryData(filename):
    # Convert digital data to binary format
    with open(filename, 'rb') as file:
        blobData = file.read()
    return blobData

def insertBLOB(prd_name, prd_picture):
    try:
        sqliteConnection = sqlite3.connect('datenbank.db')
        cursor = sqliteConnection.cursor()
        print("Connected to SQLite")
        sqlite_insert_blob_query = """ update Produkte set PRD_PICTURE = ? where PRD_NAME = ? """

        empPhoto = convertToBinaryData(prd_picture)
        # Convert data into tuple format
        data_tuple = (prd_picture, prd_name)
        cursor.execute(sqlite_insert_blob_query, data_tuple)
        sqliteConnection.commit()
        print("Image and file inserted successfully as a BLOB into a table")
        cursor.close()

    except sqlite3.Error as error:
        print("Failed to insert blob data into sqlite table", error)
    finally:
        if sqliteConnection:
            sqliteConnection.close()
            print("the sqlite connection is closed")


walker = walk("Bilder")
for root, dirs, files in walker:
    for file in files:
        print(f"\nImage: {file}")
        print(file[:-4])
        print(f"{root}/{file}")
        insertBLOB(file[:-4], f"{root}/{file}")
