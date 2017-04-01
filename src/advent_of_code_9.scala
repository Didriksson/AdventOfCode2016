import scala.io.Source

object ComputeString extends App {
  val stringToDecode = {
    val filepath = "/home/didde/workspace/AoC9/src/data.txt"
    Source.fromFile(filepath).getLines.mkString
  }.replaceAll("\\s", "")
 println(time { decodeString(stringToDecode) })
  
  def decodeString(string: String): Long = {
    val marker_re = """[(](\d+)x(\d+)[)]""".r
    marker_re.findFirstMatchIn(string) match {
      case None => string.length()
      case Some(m) => {
        val addedWithMarker = m.start + decodeString(string.drop(m.end).take(m.group(1).toInt) * m.group(2).toInt)
        val newString = string.substring((m.end) + m.group(1).toInt)
        addedWithMarker + decodeString(newString)
      }
    }

  }

  def time[R](block: => R): R = {
    val t0 = System.nanoTime()
    val result = block // call-by-name
    val t1 = System.nanoTime()
    println("Elapsed time: " + (t1 - t0) + "ns")
    result
  }

}



