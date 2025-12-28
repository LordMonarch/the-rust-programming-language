# Dangling References
Wenn ein Pointer auf einen Speicherbereich zeigt und der Speicher dann freigegeben wird, zeigt der Pointer auf einen leeren Speicherbereich. Er ist damit eine *Dangling Reference*.

In Rust ist so etwas nicht möglich, da der Wert im Speicher solange existiert wie die Referenz auf diesen Wert (sie ist valide).