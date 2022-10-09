#!/bin/bash
for row in $(python scraper/rewe.py scraper/rewe-obst-gemuese-1.html | jq -r '.[] | @base64'); do
	_jq() {
		echo ${row} | base64 --decode | jq -r ${1}
	}
	TITLE=$(_jq '.title')
	CHOSEN=$((echo "select PRD_Name from Produkte" | sqlite3 Datenbank/datenbank.db; echo "other")  | gum filter --placeholder="$TITLE")

	if [ "$CHOSEN" = "" ]
	then
		CHOSEN=$(gum input --placeholder="$TITLE")
		KATEGORIE=$(gum input --placeholder="Kategorie for $TITLE")
		echo "insert or ignore into Kategorien (KAT_NAME) values ('$KATEGORIE');" | sqlite3 Datenbank/datenbank.db
		echo "insert into Produkte (PRD_NAME, PRD_KAT_NAME) values ('$CHOSEN', '$KATEGORIE');" | sqlite3 Datenbank/datenbank.db
	fi
	echo $CHOSEN;
	PRICE=$(_jq '.grammage')

	HERKUNFT=$(echo "konventionell")
	if [ $(_jq '.organic') = 'true' ]
	then 
		HERKUNFT="bio"
	fi

	echo "insert into Produktpreise (PPR_PRD_NAME, PPR_PREIS, PPR_MENGE, PPR_EINHEIT, PPR_HERKUNFT) values ('$CHOSEN', '$PRICE', '100','g', '$HERKUNFT');" | sqlite3 Datenbank/datenbank.db
	done