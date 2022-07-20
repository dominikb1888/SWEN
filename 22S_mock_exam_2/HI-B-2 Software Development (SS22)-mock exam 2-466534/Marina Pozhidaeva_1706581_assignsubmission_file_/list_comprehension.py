
def double_list_100(number):
    if not number:
        double_list = [item * 2 for item in range(1001)]
    else: 
        double_list = [item * 2 for item in range(number + 1)]
    return double_list    


    """ This function doubles a list of numbers based on a maximum number given, if no list is given it doubles all numbers until 1000 """


    '''double_numbers = []
    numbers = range(number+1)
    for number in numbers:
        double_numbers.append(number * 2)

    return double_numbers'''
        


