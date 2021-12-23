
expression = '[][()({}[])]'

stringObject = {
    0: ['(' , ')'],
    1: ['{', "}"],
    2: ['[',']']
}


def typeOfBracket(s):
    for i in stringObject.keys():
        for j in stringObject[i]:
            if s == j:
                return i
flag = True

def check(expression):
    for i, e in enumerate(expression):
        #print(typeOfBracket(e))
        typeOf = typeOfBracket(e)

        if e == stringObject[typeOf][0] and expression[i + 1] == stringObject[typeOf][1]:
            strObj = expression[0: i:] + expression[i + 2::]
            print(strObj)
            if(strObj) == '':
                print("Reached")
                return True
                check(strObj)
    return False




print(check(expression))