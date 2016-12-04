with open("puzzleinput.txt") as f:
    content = f.readlines()
    
    
numberOfCorrect = 0
for line in content:
    sides = line.strip("\n").split()
    correctTriangleSide1 = int(sides[0]) + int(sides[1]) > int(sides[2])
    correctTriangleSide2 = int(sides[1]) + int(sides[2]) > int(sides[0])
    correctTriangleSide3 = int(sides[2]) + int(sides[0]) > int(sides[1])
    if correctTriangleSide1 and correctTriangleSide2 and correctTriangleSide3:
        numberOfCorrect = numberOfCorrect + 1
        
print "Correct number of triangles: " + str(numberOfCorrect)


numberOfCorrect = 0
row1 = []
row2 = []
row3 = []

for line in content:
    sides = line.strip("\n").split()
    row1.append(sides[0])
    row2.append(sides[1])
    row3.append(sides[2])
row1 = list(reversed(row1))
row2 = list(reversed(row2))
row3 = list(reversed(row3))

while row1:
    side1 = int(row1.pop())
    side2 = int(row1.pop())
    side3 = int(row1.pop())
    
    correctTriangleSide1 = side1 + side2 > side3
    correctTriangleSide2 = side2 + side3 > side1
    correctTriangleSide3 = side3 + side1 > side2
    if correctTriangleSide1 and correctTriangleSide2 and correctTriangleSide3:
        numberOfCorrect = numberOfCorrect + 1

while row2:
    side1 = int(row2.pop())
    side2 = int(row2.pop())
    side3 = int(row2.pop())
    
    correctTriangleSide1 = side1 + side2 > side3
    correctTriangleSide2 = side2 + side3 > side1
    correctTriangleSide3 = side3 + side1 > side2
    if correctTriangleSide1 and correctTriangleSide2 and correctTriangleSide3:
        numberOfCorrect = numberOfCorrect + 1

while row3:
    side1 = int(row3.pop())
    side2 = int(row3.pop())
    side3 = int(row3.pop())
    
    correctTriangleSide1 = side1 + side2 > side3
    correctTriangleSide2 = side2 + side3 > side1
    correctTriangleSide3 = side3 + side1 > side2
    if correctTriangleSide1 and correctTriangleSide2 and correctTriangleSide3:
        numberOfCorrect = numberOfCorrect + 1

print "Stjarna tva korrekt?: " + str(numberOfCorrect)

