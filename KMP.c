/**
 * hiho 1015	KMP�㷨
 * KMP�㷨�Ĺؼ�����ģʽ��
 */
#include <stdio.h>
#include <string.h>

#define PLEN 1000000
#define TLEN 10000
void getNext(char T[], int next[])
{
    int j= -1, i=0;
    next[0] = -1;
    while(i < strlen(T))
    {
        if(j == -1 || T[i] == T[j])
        {
            i++;
            j++;
            next[i] = j;
            if(T[i] != T[j])
            {
                next[i] = j;
            }
            else
            {
                next[i] = next[j];
            }
        }
        else
        {
            // j ����
            // ǰ׺�ǹ̶��ģ���׺����Ե�
            j = next[j];
        }
    }
}

/**
 *
 * @param P ԭ��
 * @param T ģʽ��
 * @return
 */
int index_KMP(char P[], char T[])
{
    int i, j, count, lenp, lent;

    i = 0;
    j = 0;
    count = 0;
    lenp = strlen(P);
    lent = strlen(T);
    int next[TLEN] = {-1};
    getNext(T, next);
    while(i < lenp)
    {
        if(j == -1 || P[i] == T[j])
        {
            i++;
            j++;
        }
        else
        {
            j = next[j];
        }

        if(j == lent)
        {
            count++;
        }
    }
    return count;
}

int main() {
    int m;
    char P[PLEN+1], T[TLEN+1];
    scanf("%d", &m);
    while(m--)
    {
        scanf("%s", T);
        scanf("%s", P);
        printf("%d\n", index_KMP(P, T));
    }
    return 0;
}