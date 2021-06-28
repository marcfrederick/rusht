# Rusht Projektbericht

In unserem Projekt haben wir einen Interpreter für einen eigenen Lisp-Dialekt "Rusht" realisiert. Ziel war es, dass der
Interpreter Lisp-Code mittels einer REPL (Read-Eval-Print-Loop) einliest und interpretiert.

Hierbei war es uns wichtig nicht einfach auf Bibliotheken, die die Generation des Parsers übernehmen, zurückzugreifen,
sondern jeden Schritt vom Tokenizen über das Parsen bis hin zum Interpretieren selber zu implementieren.

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

Bei der Implementierung von Lambda-Funktionen sind wir auf ein Problem mit unserer Implementierung des Parsers gestoßen,
die größere Änderungen am bestehenden Code erforderten. Insbesondere war eine Änderung an der Art und Weise, wie wir die
beim Parsen erstellten Ausdrücke speichern, erforderlich.

Zuvor hatten wir hier lediglich die bereits beim Tokenizen generierten Tokens in einem `Expr` enum gespeichert und
wiederverwendet. Sämtliche in der Prelude definierten Funktionen hatten den Typ `fn(Vec<Token>) -> Result<Token, Error>`
.
