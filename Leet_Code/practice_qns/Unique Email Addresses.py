class Solution:
    def numUniqueEmails(self, emails):
        sorted_emails = []
        for i in emails:
            split_name_email = i.split('@')
            split_name_email[0] = self.simplifyName(split_name_email[0])
            simplified_email = '@'.join(split_name_email)
            sorted_emails.append(simplified_email)
        # print(sorted_emails)
        unique_emails = 0
        while(len(sorted_emails) > 0):
            temp_to_check = sorted_emails.pop()
            unique_emails += 1
            counter = len(sorted_emails) - 1
            while(counter > -1):
                if(temp_to_check == sorted_emails[counter]):
                    sorted_emails.pop(counter)
                counter -= 1
        return unique_emails


    def simplifyName(self, name):
        first_cut = name.split('+')[0]
        second_cut = first_cut.split('.')
        name_to_rtr = ''
        for i in second_cut:
            name_to_rtr += i
        return name_to_rtr

a = Solution()
print(a.numUniqueEmails(["test.email+alex@leetcode.com","test.e.mail+bob.cathy@leetcode.com","testemail+david@lee.tcode.com"]))