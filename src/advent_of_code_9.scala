import scala.io.Source


object ComputeString extends App {
  val stringToDecode = {
  val filepath = "absolute/Path/To/Data.txt"
  Source.fromFile(filepath).getLines.mkString
}
  println(decodeString(stringToDecode).length())
      
  def decodeString(string: String): String = {
     val opMarkerIndex = getMarkerLocation(string)
     if(opMarkerIndex.isEmpty){
       string
     }
     else{
        val indexOfMarker = opMarkerIndex.get
        val marker = string.substring(indexOfMarker._1, indexOfMarker._2).split("x").map(_.toInt)
        string.take(indexOfMarker._1-1) + string.drop(indexOfMarker._2+1).take(marker(0))*marker(1) + decodeString(string.substring((indexOfMarker._2 + 1  + marker(0))))
     }
   }
  
    def getMarkerLocation(string: String) = {
     val startOfMarker = string.indexOf('(') + 1
     val endOfMarker = string.indexOf(')')
     if(startOfMarker == -1 || endOfMarker == -1)
       None
     else 
       Some((startOfMarker, endOfMarker))
   }

}



