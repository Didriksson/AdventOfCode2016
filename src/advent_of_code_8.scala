import scala.io.Source

object Adventofcode8 extends App {

  var screen = Array.ofDim[Boolean](6, 50)

  val inputdata = {
    val filepath = "/home/didde/workspace/AoC9/src/data.txt"
    Source.fromFile(filepath).getLines.toList
  }

  def printScreen = {
    screen.foreach { row => {
      row.foreach{ v =>
         if(v){
           print("O")
         }
         else{
           print(".")
         }
      }      
      println 
      }
    }
  }

  def handleInput(command: String) = {
    """"""
    command match {
      case c if command.startsWith("rect") => {
        val Array(w, h) = c.split(" ").tail.mkString.split('x')
        drawRectangle(w.toInt, h.toInt);
      }
      case c if command.startsWith("rotate column") => {
        val Array(col, n) = c.dropWhile(_ != '=').drop(1).split(" by ")
        rotateColumn(col.toInt, n.toInt)
      }
      case c if command.startsWith("rotate row") => {
        val Array(row, n) = c.dropWhile(_ != '=').drop(1).split(" by ")
        rotateRow(row.toInt, n.toInt)
      }

      case _ => println("Unknown command: " + command)
    }
  }

  def drawRectangle(w: Int, h: Int): Unit = {
    for (x <- 0 until w) {
      for (y <- 0 until h) {
        screen(y)(x) = true;
      }
    }
  }

  def rotateColumn(col: Int, n: Int) = {
    var previousState = Array.ofDim[Boolean](screen.length)
    for (y <- 0 until screen.length) {
      previousState(y) = screen(y)(col);
    }

    for (i <- 0 until previousState.length) {
      screen(i)(col) = previousState(Math.floorMod(i - n, previousState.length))
    }

  }

  def rotateRow(row: Int, n: Int) = {
    var previousState = screen(row).clone();

    for (i <- 0 until previousState.length) {
      screen(row)(i) = previousState(Math.floorMod(i - n, previousState.length))
    }
  }
  
  inputdata.foreach(f => handleInput(f))
  println(printScreen)
  def countPixelsLit: Int = {
    val flatscreen = screen.flatten
    flatscreen.count(_.booleanValue())
  }
}



