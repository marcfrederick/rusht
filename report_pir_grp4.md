# Rusht Projektbericht

In unserem Projekt haben wir einen Interpreter für einen eigenen Lisp-Dialekt "Rusht" realisiert. Ziel war es, dass der
Interpreter Lisp-Code mittels einer REPL (Read-Eval-Print-Loop) einliest und interpretiert.

Hierbei war es uns wichtig nicht einfach auf Bibliotheken, die die Generation des Parsers übernehmen, zurückzugreifen,
sondern jeden Schritt vom Tokenizen über das Parsen bis hin zum Interpretieren selber zu implementieren.

Weiter Informationen über das Projekt sind in der `README.md` auffindbar.

## Was haben Sie bei der Realisierung gelernt?

Im Rahmen unseres Projektes haben wir uns intensiv mit der Implementierung von Parsern und Interpretern beschäftigt.
Dies war für uns beide ein neues Thema und erforderte einiges an Einarbeitung, bevor wir mit der Umsetzung des Projektes
beginnen konnten.

Weiterhin mussten wir uns mit der Familie der Lisps beschäftigen, um bei der Implementierung unseres eigenen nicht von
null anzufangen.

## Haben Sie Prioritäten für Ihre Tasks vergeben, wenn ja, welche und warum?

Die Prioritäten haben wir nach der bekannten Prioritätenvergabe sortiert (von hoch zu niedrig):

1. tokenizer, parser, interpreter:\
   Damit der Lisp-Interpreter überhaupt funktioniert, mussten die drei Schritte jeweils umgesetzt werden. Somit hatten
   sie die höchste Priorität für die Umsetzung unseres Projekts.
2. Einfache eingebaute mathematische Funktionen (+, -):\
   Um erste Versuche mit unserem Interpreter machen zu können, war es uns wichtig mit mathematischen Operationen zu
   beginnen und auf diesen aufzubauen.
3. Variablen:\
   Als besonderes erstes Feature wollten wir die Möglichkeit einer eigenen Variablen Deklaration nutzen. Darunter ist zu
   verstehen, dass die REPL effizient genutzt werden kann und bereits berechnete Ergebnisse in einer weiteren Berechnung
   wieder verwendet werden können. Dies erspart außerdem Wiederholungen und einer besseren Nutzung und Anwendung unseres
   Interpreters.
4. Standardbibliothek (Prelude):
   Die jeweils geschriebenen Funktionen (`concat`, `exit`, `add`, etc.) wurden in unserer Bibliothek geschrieben und
   umgesetzt. Auch gilt dies als Erweiterung von Features und somit auch dem Ausbau unseres Interpreters.
5. Lambda-Funktionen:\
   Als End-Feature wollten wir außerdem eigene Funktionen definieren können. Für die Umsetzung wurde ein Lambda Ausdruck
   eingebaut. Dies ermöglicht wiederum Wiederholungen zu umgehen und die Effizienz und Abwechslung zu steigern.

## Haben Sie Ihre Tasks geplant, wenn ja, in welchen Zeitabständen führen Sie sie aus?

Damit wir unseren Lisp-Interpreter gut und effizient umsetzen konnten, hatten wir zweimal in der Woche ein Meeting. Was
darauf basierte, dass wir unsere jeweiligen Schritte vorführen und erklären konnten. Falls Probleme während der
Bearbeitung aufgetreten sind, haben wir uns kontaktiert, da wir sowieso im ständigen Kontakt waren. Spontane Treffen,
wir eine kurze Abend-Session, haben auch öfters stattgefunden. Je nachdem wie der andere Partner zeitlich Zeit hatte.
Zusammen haben via Code-With-Me gearbeitet, da wir zusammen an dem Projekt arbeiten wollten.

## Gehen Sie auf Schwierigkeiten ein, sofern es welche gab.

### Fehlerbehandlung

Bei der Programmierung des ursprünglichen Prototyps des Rusht Interpreters hatten wir anfangs wenig Priorität auf
korrekte Fehlerbehandlung gelegt. Daher hatten wir an vielen Stellen im Code Aufrufe an `panic!()`, `unwrap()`,
und `expect()`. Das führte bei der Implementierung der REPL zu Problemen, da jede falsche Eingabe zu einer Panik und
damit auch zur Beendigung der REPL führte. Daher hatten wir an dieser Stelle versucht diese Paniken mit
der `std::panic::catch_unwind` Funktion abzufangen und zu behandeln, aber es wurde klar, dass wir auf echte
Fehlerbehandlung und den damit einhergehenden `Result` Datentypen umbauen mussten.

Der Umbau an sich erforderte große Änderungen an der Code-Basis und war sehr zeitaufwändig. Würden wir das Projekt
nochmal durchführen, würden wir sicherstellen, dass wir von vornherein Fehler ordentlich behandeln, da sich die
initialen Zeitersparnisse später rächen.

### Interpreter

Weiterhin sind wir bei der Implementierung der Lambda-Funktionen auf ein Problem mit der Implementierung unseres
Interpreters und der Prelude gestoßen, die ebenfalls größere Änderungen am bestehenden Code erforderten. Funktionen
waren derart konzipiert, dass diese die Signatur `fn(Vec<Token>) -> Result<Token, Error>` hatten. Dies führte bei der
implementierung von Lambda ausdrücken zu Problemen, da die Variablendefinition ebenfalls nur mit Elementen vom
Typ `Token`, die also schon beim Tokenizen bekannt waren, funktionierte.

Um dieses Problem zu beheben, haben wir unseren Code derart umgeschrieben, dass sämtlicher Code außerhalb des Tokenizers
selber mit Elementen vom `Expr`-Typen arbeiten.
