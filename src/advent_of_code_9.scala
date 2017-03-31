import scala.io.Source

object ComputeString extends App {
  val stringToDecode = {
    val filepath = "/home/didde/workspace/AoC9/src/data.txt"
    Source.fromFile(filepath).getLines.mkString
  }.replaceAll("\\s", "")
  println(decodeString(stringToDecode, 0))

  def decodeString(string: String, length: Int): Int = {
    println(string.length())
    val opMarkerIndex = getMarkerLocation(string)
    if (opMarkerIndex.isEmpty) {
      length + string.length()
    } else {
      val indexOfMarker = opMarkerIndex.get
      val marker = string.substring(indexOfMarker._1, indexOfMarker._2).split("x").map(_.toInt)
      val addedWithMarker = string.take(indexOfMarker._1 - 1) + string.drop(indexOfMarker._2 + 1).take(marker(0)) * marker(1)
      val newString = addedWithMarker + string.substring((indexOfMarker._2 + 1) + marker(0))
      decodeString(newString.drop(indexOfMarker._1 - 1), length + indexOfMarker._1-1)
    }
  }

  def getMarkerLocation(string: String) = {
    val startOfMarker = string.indexOf('(') + 1
    val endOfMarker = string.indexOf(')')
    if (startOfMarker == -1 || endOfMarker == -1)
      None
    else
      Some((startOfMarker, endOfMarker))
  }

}



