package didriksson

import scala.io.Source

abstract class RobotReceivable(val id: String) {
  def giveValue(value: Int): RobotReceivable;

  def giveValue(value: Option[Int]): RobotReceivable = {
    if (value.isDefined) {
      this.giveValue(value.get)
    } else {
      this
    }
  }

  def isReadyForAction = false
}

class Output(id: String, val outputs: List[Int] = List()) extends RobotReceivable(id) {

  def giveValue(value: Int): Output = {
    new Output(id, outputs.+:(value));
  }

  override def toString: String = {
    "[" + id + ": " + outputs.mkString + "]"
  }

}

class Robot(id: String, val low: Option[Int] = None, val high: Option[Int] = None) extends RobotReceivable(id) {
  if (low.getOrElse(-1) == 17 && high.getOrElse(-1) == 61)
    println(this)

  def giveValue(value: Int): Robot = {
    if (low.exists(p => p < value)) {
      new Robot(id, low, Some(value))
    } else {
      new Robot(id, Some(value), low)
    }
  }

  override def isReadyForAction = low.isDefined && high.isDefined

  override def toString: String = {
    "[Robot: " + id + ". Low: " + low.getOrElse(None) + " High: " + high.getOrElse(None) + "]"
  }
}

object Adventofcode10 extends App {
  val inputdata = {
    val filepath = "/home/didde/workspace/AoC9/src/data.txt"
    Source.fromFile(filepath).getLines.toList
  }

  def handleCommands(c: String, robots: Map[String, RobotReceivable]): Map[String, RobotReceivable] = {
    c match {
      case c if c.startsWith("value") => {
        val Array(value, bot) = c.split("value ").tail.mkString.split(" goes to ")
        giveValue(bot, value.toInt, robots)
      }

      case c if c.startsWith("bot ") => {
        val sourceBot = c.split(" gives low to ").head
        val Array(lowTo, highTo) = c.split(" gives low to ").tail(0).split(" and high to ")
        botGiveAway(sourceBot, lowTo, highTo, robots);
      }

      case _ => throw new Exception("Unknown command: " + c);
    }

  }
  def giveValue(bot: String, value: Int, robots: Map[String, RobotReceivable]): Map[String, RobotReceivable] = {
    val r = robots.getOrElse(bot, {
      if (bot.startsWith("output")) {
        new Output(bot)
      } else {
        new Robot(bot)
      }
    })
    robots.+((bot, r.giveValue(value)))
  }

  def botGiveAway(sourceBot: String, lowTo: String, highTo: String, robots: Map[String, RobotReceivable]): Map[String, RobotReceivable] = {
    val source = robots.getOrElse(sourceBot, new Robot(sourceBot)).asInstanceOf[Robot]
    if (source.low.isEmpty || source.high.isEmpty) {
      robots
    } else {
      val lowBot = robots.getOrElse(lowTo, {
        if (lowTo.startsWith("output")) {
          new Output(lowTo)
        } else {
          new Robot(lowTo)
        }
      })
      val highBot = robots.getOrElse(highTo, {
        if (highTo.startsWith("output")) {
          new Output(highTo)
        } else {
          new Robot(highTo)
        }
      })

      val newLow = lowBot.giveValue(source.low);
      val newHigh = highBot.giveValue(source.high);
      val newSource = new Robot(sourceBot)
      robots.+((newLow.id, newLow)).+((newHigh.id, newHigh)).+((newSource.id, newSource))
    }
  }

  var m: Map[String, RobotReceivable] = Map.empty[String, RobotReceivable];
  inputdata.foreach(p => {
    if (p.startsWith("value ")) {
      m = handleCommands(p, m);
    }
  })
  var commands = inputdata.filter(_.startsWith("bot ")).toList;
  while (m.values.exists(_.isReadyForAction)) {
    commands.foreach(p => m = handleCommands(p, m))
  }
  println("Part 2 answer: " + m.values.filter(p => p.id == "output 0" || p.id == "output 1" || p.id == "output 2").map(_.asInstanceOf[Output]).flatMap(p => p.outputs).product)

}


