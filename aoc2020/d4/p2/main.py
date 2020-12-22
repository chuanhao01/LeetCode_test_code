import re

class Passport:
    def __init__(self, byr=None, iyr=None, eyr=None, hgt=None, hcl=None, ecl=None, pid=None, cid=None):
        self.byr = byr
        self.iyr = iyr
        self.eyr = eyr
        self.hgt = hgt
        self.hcl = hcl
        self.ecl = ecl
        self.pid = pid
        self.cid = cid
    
    def check_valid(self):
        '''
        Checks if the current passport is valid
        Returns the bool
        '''
        val = True
        val  = val and self.__check_byr()
        val  = val and self.__check_iyr()
        val  = val and self.__check_eyr()
        val  = val and self.__check_hgt()
        val  = val and self.__check_hcl()
        val  = val and self.__check_ecl()
        val  = val and self.__check_pid()
        val  = val and self.__check_cid()
        return val

    def __check_byr(self):
        '''
        Helper func to check if the byr is valid
        '''
        try:
            byr = int(self.byr)
        except:
            return False
        return 1920 <= byr <= 2002

    def __check_iyr(self):
        '''
        Helper func to check if the iyr is valid
        '''
        try:
            iyr = int(self.iyr)
        except:
            return False
        return 2010 <= iyr <= 2020
    
    def __check_eyr(self):
        '''
        Helper func to check if the eyr is valid
        '''
        try:
            eyr = int(self.eyr)
        except:
            return False
        return 2020 <= eyr <= 2030
    
    def __check_hgt(self):
        '''
        Helper func to check if the hgt is valid
        '''
        if self.hgt is None:
            return False
        hgt_format = self.hgt[-2:]
        try:
            hgt = int(self.hgt[:-2])
        except:
            return False

        if hgt_format == 'cm':
            return 150 <= hgt <= 193
        elif hgt_format == 'in':
            return 59 <= hgt <= 76
        else:
            return False
    
    def __check_hcl(self):
        if self.hcl is None:
            return False
        return bool(re.match(r"#[0-9a-f]{6}", self.hcl))
    
    def __check_ecl(self):
        if self.ecl is None:
            return False
        check_set = {"amb", "blu", "brn", "gry", "grn", "hzl", "oth"} 
        if self.ecl in check_set:
            return True
        return False
    
    def __check_pid(self):
        if self.pid is None:
            return False
        try:
            _ = int(self.pid) # Check if int
            return len(self.pid) == 9
        except:
            return False
    
    def __check_cid(self):
        return True
        

# a = {
#     'byr': '2002',
#     'iyr': '2012',
#     'eyr': '2022',
#     'hgt': '60in',
#     'hcl': '#123abc',
#     'ecl': 'brn',
#     'pid': '000000021'
# }
# a = Passport(**a)
# print(a.check_valid())


total = 0
valids = 0
passports = []
with open('data.txt') as f:
    cur_pass = {}
    for l in f:
        l = l.rstrip()
        if l == '':
            passports.append(Passport(**cur_pass))
            cur_pass = {}
            continue
        f = [i.split(':') for i in l.split(' ')]
        for k, v in f:
            cur_pass[k] = v
    # For the last one
    passports.append(Passport(**cur_pass))
    for p in passports:
        if p.check_valid():
            valids += 1
print(valids)
print(total)
        