class Solution:
    def defangIPaddr(self, address):
        address = address.split(".")
        new_addr = ''
        for i in range(len(address)):
            if(i<len(address) - 1):
                new_addr += address[i] + '[.]'
            else:
                new_addr += address[i]
        return new_addr