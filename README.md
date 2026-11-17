# meal_counter - Essensgeld Zähler
Ein kleines Projekt von mir, dass auf meinem Termux ausgeführt wird und 
die Tage, die unser kurzer bei der Tagesmutter ist, zählt und gesammtkosten für das Essen berechnet.
Absolut overkill das so zu schreiben, ist mehr ein learning-rust-by-doing Ding.

## Usage:
| Option | Usage |
|- |- |   
| none |   Tag Incrementieren          |
| -a   |   Tag manuell Incrementieren  |
| -h   |   help    |
| -m   |   Minus 1 Tag |
| -s x |   Auf Tag x setzen    |
| -i   |   Tage anzeigen   |
| -r   |   Tage auf 0 zurück setzen    |

## Kosten Info:
Die Kosten (value) sind Hard auf 2.50 Codiert (Siehe Lib.rs => PRICE) und werden mit der gegebenen Tagesanzahl (days) multiplizert.

## Tag erhöhung
Ohne Argument, wird der Tag bei ausführung einmal täglich um einen Tag erhöht.
Möchte man den Tag Counter dann nochmal verändern, muss dies per Argument geschehen.
Eine negativ anzahl von Tagen wird nicht gespeichert.