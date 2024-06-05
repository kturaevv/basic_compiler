#include <stdio.h>
int main(void){
printf("How many fibonacci numbers do you want?\n");
int nums;
scanf("%d", &nums);
printf("\n");
int a = 0;
int b = 1;
while ( nums > 0) {
printf("%d\n",  a);

int c =  a +  b;

 a =  b;

 b =  c;

 nums =  nums - 1;



}
return 0;
}
