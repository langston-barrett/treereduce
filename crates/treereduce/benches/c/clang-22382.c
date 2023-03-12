typedef unsigned int size_t;
typedef signed char int8_t;
typedef short int int16_t;
typedef int int32_t;
typedef long long int int64_t;
typedef unsigned char uint8_t;
typedef unsigned short int uint16_t;
typedef unsigned int uint32_t;
typedef unsigned long long int uint64_t;
int printf (const char *, ...);
void __assert_fail (const char *__assertion, const char *__file, unsigned int __line, const char *__function);
static void
platform_main_end(uint32_t crc, int flag)
{





                                 
}
static int8_t
(safe_unary_minus_func_int8_t_s)(int8_t si )
{

  return






    -si;
}

static int8_t
(safe_add_func_int8_t_s_s)(int8_t si1, int8_t si2 )
{

  return






    (si1 + si2);
}

static int8_t
(safe_sub_func_int8_t_s_s)(int8_t si1, int8_t si2 )
{

  return






    (si1 - si2);
}

static int8_t
(safe_mul_func_int8_t_s_s)(int8_t si1, int8_t si2 )
{

  return






    si1 * si2;
}

static int8_t
(safe_mod_func_int8_t_s_s)(int8_t si1, int8_t si2 )
{

  return

    ((si2 == 0) || ((si1 == (-128)) && (si2 == (-1)))) ?
    ((si1)) :

    (si1 % si2);
}

static int8_t
(safe_div_func_int8_t_s_s)(int8_t si1, int8_t si2 )
{

  return

    ((si2 == 0) || ((si1 == (-128)) && (si2 == (-1)))) ?
    ((si1)) :

    (si1 / si2);
}

static int8_t
(safe_lshift_func_int8_t_s_s)(int8_t left, int right )
{

  return

    ((left < 0) || (((int)right) < 0) || (((int)right) >= 32) || (left > ((127) >> ((int)right)))) ?
    ((left)) :

    (left << ((int)right));
}

static int8_t
(safe_lshift_func_int8_t_s_u)(int8_t left, unsigned int right )
{

  return

    ((left < 0) || (((unsigned int)right) >= 32) || (left > ((127) >> ((unsigned int)right)))) ?
    ((left)) :

    (left << ((unsigned int)right));
}

static int8_t
(safe_rshift_func_int8_t_s_s)(int8_t left, int right )
{

  return

    ((left < 0) || (((int)right) < 0) || (((int)right) >= 32))?
    ((left)) :

    (left >> ((int)right));
}

static int8_t
(safe_rshift_func_int8_t_s_u)(int8_t left, unsigned int right )
{

  return

    ((left < 0) || (((unsigned int)right) >= 32)) ?
    ((left)) :

    (left >> ((unsigned int)right));
}



static int16_t
(safe_unary_minus_func_int16_t_s)(int16_t si )
{

  return






    -si;
}

static int16_t
(safe_add_func_int16_t_s_s)(int16_t si1, int16_t si2 )
{

  return






    (si1 + si2);
}

static int16_t
(safe_sub_func_int16_t_s_s)(int16_t si1, int16_t si2 )
{

  return






    (si1 - si2);
}

static int16_t
(safe_mul_func_int16_t_s_s)(int16_t si1, int16_t si2 )
{

  return






    si1 * si2;
}

static int16_t
(safe_mod_func_int16_t_s_s)(int16_t si1, int16_t si2 )
{

  return

    ((si2 == 0) || ((si1 == (-32767-1)) && (si2 == (-1)))) ?
    ((si1)) :

    (si1 % si2);
}

static int16_t
(safe_div_func_int16_t_s_s)(int16_t si1, int16_t si2 )
{

  return

    ((si2 == 0) || ((si1 == (-32767-1)) && (si2 == (-1)))) ?
    ((si1)) :

    (si1 / si2);
}

static int16_t
(safe_lshift_func_int16_t_s_s)(int16_t left, int right )
{

  return

    ((left < 0) || (((int)right) < 0) || (((int)right) >= 32) || (left > ((32767) >> ((int)right)))) ?
    ((left)) :

    (left << ((int)right));
}

static int16_t
(safe_lshift_func_int16_t_s_u)(int16_t left, unsigned int right )
{

  return

    ((left < 0) || (((unsigned int)right) >= 32) || (left > ((32767) >> ((unsigned int)right)))) ?
    ((left)) :

    (left << ((unsigned int)right));
}

static int16_t
(safe_rshift_func_int16_t_s_s)(int16_t left, int right )
{

  return

    ((left < 0) || (((int)right) < 0) || (((int)right) >= 32))?
    ((left)) :

    (left >> ((int)right));
}

static int16_t
(safe_rshift_func_int16_t_s_u)(int16_t left, unsigned int right )
{

  return

    ((left < 0) || (((unsigned int)right) >= 32)) ?
    ((left)) :

    (left >> ((unsigned int)right));
}



static int32_t
(safe_unary_minus_func_int32_t_s)(int32_t si )
{

  return


    (si==(-2147483647-1)) ?
    ((si)) :


    -si;
}

static int32_t
(safe_add_func_int32_t_s_s)(int32_t si1, int32_t si2 )
{

  return


    (((si1>0) && (si2>0) && (si1 > ((2147483647)-si2))) || ((si1<0) && (si2<0) && (si1 < ((-2147483647-1)-si2)))) ?
    ((si1)) :


    (si1 + si2);
}

static int32_t
(safe_sub_func_int32_t_s_s)(int32_t si1, int32_t si2 )
{

  return


    (((si1^si2) & (((si1 ^ ((si1^si2) & (~(2147483647))))-si2)^si2)) < 0) ?
    ((si1)) :


    (si1 - si2);
}

static int32_t
(safe_mul_func_int32_t_s_s)(int32_t si1, int32_t si2 )
{

  return


    (((si1 > 0) && (si2 > 0) && (si1 > ((2147483647) / si2))) || ((si1 > 0) && (si2 <= 0) && (si2 < ((-2147483647-1) / si1))) || ((si1 <= 0) && (si2 > 0) && (si1 < ((-2147483647-1) / si2))) || ((si1 <= 0) && (si2 <= 0) && (si1 != 0) && (si2 < ((2147483647) / si1)))) ?
    ((si1)) :


    si1 * si2;
}

static int32_t
(safe_mod_func_int32_t_s_s)(int32_t si1, int32_t si2 )
{

  return

    ((si2 == 0) || ((si1 == (-2147483647-1)) && (si2 == (-1)))) ?
    ((si1)) :

    (si1 % si2);
}

static int32_t
(safe_div_func_int32_t_s_s)(int32_t si1, int32_t si2 )
{

  return

    ((si2 == 0) || ((si1 == (-2147483647-1)) && (si2 == (-1)))) ?
    ((si1)) :

    (si1 / si2);
}

static int32_t
(safe_lshift_func_int32_t_s_s)(int32_t left, int right )
{

  return

    ((left < 0) || (((int)right) < 0) || (((int)right) >= 32) || (left > ((2147483647) >> ((int)right)))) ?
    ((left)) :

    (left << ((int)right));
}

static int32_t
(safe_lshift_func_int32_t_s_u)(int32_t left, unsigned int right )
{

  return

    ((left < 0) || (((unsigned int)right) >= 32) || (left > ((2147483647) >> ((unsigned int)right)))) ?
    ((left)) :

    (left << ((unsigned int)right));
}

static int32_t
(safe_rshift_func_int32_t_s_s)(int32_t left, int right )
{

  return

    ((left < 0) || (((int)right) < 0) || (((int)right) >= 32))?
    ((left)) :

    (left >> ((int)right));
}

static int32_t
(safe_rshift_func_int32_t_s_u)(int32_t left, unsigned int right )
{

  return

    ((left < 0) || (((unsigned int)right) >= 32)) ?
    ((left)) :

    (left >> ((unsigned int)right));
}




static int64_t
(safe_unary_minus_func_int64_t_s)(int64_t si )
{

  return


    (si==(-9223372036854775807L -1)) ?
    ((si)) :


    -si;
}

static int64_t
(safe_add_func_int64_t_s_s)(int64_t si1, int64_t si2 )
{

  return


    (((si1>0) && (si2>0) && (si1 > ((9223372036854775807L)-si2))) || ((si1<0) && (si2<0) && (si1 < ((-9223372036854775807L -1)-si2)))) ?
    ((si1)) :


    (si1 + si2);
}

static int64_t
(safe_sub_func_int64_t_s_s)(int64_t si1, int64_t si2 )
{

  return


    (((si1^si2) & (((si1 ^ ((si1^si2) & (~(9223372036854775807L))))-si2)^si2)) < 0) ?
    ((si1)) :


    (si1 - si2);
}

static int64_t
(safe_mul_func_int64_t_s_s)(int64_t si1, int64_t si2 )
{

  return


    (((si1 > 0) && (si2 > 0) && (si1 > ((9223372036854775807L) / si2))) || ((si1 > 0) && (si2 <= 0) && (si2 < ((-9223372036854775807L -1) / si1))) || ((si1 <= 0) && (si2 > 0) && (si1 < ((-9223372036854775807L -1) / si2))) || ((si1 <= 0) && (si2 <= 0) && (si1 != 0) && (si2 < ((9223372036854775807L) / si1)))) ?
    ((si1)) :


    si1 * si2;
}

static int64_t
(safe_mod_func_int64_t_s_s)(int64_t si1, int64_t si2 )
{

  return

    ((si2 == 0) || ((si1 == (-9223372036854775807L -1)) && (si2 == (-1)))) ?
    ((si1)) :

    (si1 % si2);
}

static int64_t
(safe_div_func_int64_t_s_s)(int64_t si1, int64_t si2 )
{

  return

    ((si2 == 0) || ((si1 == (-9223372036854775807L -1)) && (si2 == (-1)))) ?
    ((si1)) :

    (si1 / si2);
}

static int64_t
(safe_lshift_func_int64_t_s_s)(int64_t left, int right )
{

  return

    ((left < 0) || (((int)right) < 0) || (((int)right) >= 32) || (left > ((9223372036854775807L) >> ((int)right)))) ?
    ((left)) :

    (left << ((int)right));
}

static int64_t
(safe_lshift_func_int64_t_s_u)(int64_t left, unsigned int right )
{

  return

    ((left < 0) || (((unsigned int)right) >= 32) || (left > ((9223372036854775807L) >> ((unsigned int)right)))) ?
    ((left)) :

    (left << ((unsigned int)right));
}

static int64_t
(safe_rshift_func_int64_t_s_s)(int64_t left, int right )
{

  return

    ((left < 0) || (((int)right) < 0) || (((int)right) >= 32))?
    ((left)) :

    (left >> ((int)right));
}

static int64_t
(safe_rshift_func_int64_t_s_u)(int64_t left, unsigned int right )
{

  return

    ((left < 0) || (((unsigned int)right) >= 32)) ?
    ((left)) :

    (left >> ((unsigned int)right));
}







static uint8_t
(safe_unary_minus_func_uint8_t_u)(uint8_t ui )
{

  return -ui;
}

static uint8_t
(safe_add_func_uint8_t_u_u)(uint8_t ui1, uint8_t ui2 )
{

  return ui1 + ui2;
}

static uint8_t
(safe_sub_func_uint8_t_u_u)(uint8_t ui1, uint8_t ui2 )
{

  return ui1 - ui2;
}

static uint8_t
(safe_mul_func_uint8_t_u_u)(uint8_t ui1, uint8_t ui2 )
{

  return ((unsigned int)ui1) * ((unsigned int)ui2);
}

static uint8_t
(safe_mod_func_uint8_t_u_u)(uint8_t ui1, uint8_t ui2 )
{

  return

    (ui2 == 0) ?
    ((ui1)) :

    (ui1 % ui2);
}

static uint8_t
(safe_div_func_uint8_t_u_u)(uint8_t ui1, uint8_t ui2 )
{

  return

    (ui2 == 0) ?
    ((ui1)) :

    (ui1 / ui2);
}

static uint8_t
(safe_lshift_func_uint8_t_u_s)(uint8_t left, int right )
{

  return

    ((((int)right) < 0) || (((int)right) >= 32) || (left > ((255) >> ((int)right)))) ?
    ((left)) :

    (left << ((int)right));
}

static uint8_t
(safe_lshift_func_uint8_t_u_u)(uint8_t left, unsigned int right )
{

  return

    ((((unsigned int)right) >= 32) || (left > ((255) >> ((unsigned int)right)))) ?
    ((left)) :

    (left << ((unsigned int)right));
}

static uint8_t
(safe_rshift_func_uint8_t_u_s)(uint8_t left, int right )
{

  return

    ((((int)right) < 0) || (((int)right) >= 32)) ?
    ((left)) :

    (left >> ((int)right));
}

static uint8_t
(safe_rshift_func_uint8_t_u_u)(uint8_t left, unsigned int right )
{

  return

    (((unsigned int)right) >= 32) ?
    ((left)) :

    (left >> ((unsigned int)right));
}



static uint16_t
(safe_unary_minus_func_uint16_t_u)(uint16_t ui )
{

  return -ui;
}

static uint16_t
(safe_add_func_uint16_t_u_u)(uint16_t ui1, uint16_t ui2 )
{

  return ui1 + ui2;
}

static uint16_t
(safe_sub_func_uint16_t_u_u)(uint16_t ui1, uint16_t ui2 )
{

  return ui1 - ui2;
}

static uint16_t
(safe_mul_func_uint16_t_u_u)(uint16_t ui1, uint16_t ui2 )
{

  return ((unsigned int)ui1) * ((unsigned int)ui2);
}

static uint16_t
(safe_mod_func_uint16_t_u_u)(uint16_t ui1, uint16_t ui2 )
{

  return

    (ui2 == 0) ?
    ((ui1)) :

    (ui1 % ui2);
}

static uint16_t
(safe_div_func_uint16_t_u_u)(uint16_t ui1, uint16_t ui2 )
{

  return

    (ui2 == 0) ?
    ((ui1)) :

    (ui1 / ui2);
}

static uint16_t
(safe_lshift_func_uint16_t_u_s)(uint16_t left, int right )
{

  return

    ((((int)right) < 0) || (((int)right) >= 32) || (left > ((65535) >> ((int)right)))) ?
    ((left)) :

    (left << ((int)right));
}

static uint16_t
(safe_lshift_func_uint16_t_u_u)(uint16_t left, unsigned int right )
{

  return

    ((((unsigned int)right) >= 32) || (left > ((65535) >> ((unsigned int)right)))) ?
    ((left)) :

    (left << ((unsigned int)right));
}

static uint16_t
(safe_rshift_func_uint16_t_u_s)(uint16_t left, int right )
{

  return

    ((((int)right) < 0) || (((int)right) >= 32)) ?
    ((left)) :

    (left >> ((int)right));
}

static uint16_t
(safe_rshift_func_uint16_t_u_u)(uint16_t left, unsigned int right )
{

  return

    (((unsigned int)right) >= 32) ?
    ((left)) :

    (left >> ((unsigned int)right));
}



static uint32_t
(safe_unary_minus_func_uint32_t_u)(uint32_t ui )
{

  return -ui;
}

static uint32_t
(safe_add_func_uint32_t_u_u)(uint32_t ui1, uint32_t ui2 )
{

  return ui1 + ui2;
}

static uint32_t
(safe_sub_func_uint32_t_u_u)(uint32_t ui1, uint32_t ui2 )
{

  return ui1 - ui2;
}

static uint32_t
(safe_mul_func_uint32_t_u_u)(uint32_t ui1, uint32_t ui2 )
{

  return ((unsigned int)ui1) * ((unsigned int)ui2);
}

static uint32_t
(safe_mod_func_uint32_t_u_u)(uint32_t ui1, uint32_t ui2 )
{

  return

    (ui2 == 0) ?
    ((ui1)) :

    (ui1 % ui2);
}

static uint32_t
(safe_div_func_uint32_t_u_u)(uint32_t ui1, uint32_t ui2 )
{

  return

    (ui2 == 0) ?
    ((ui1)) :

    (ui1 / ui2);
}

static uint32_t
(safe_lshift_func_uint32_t_u_s)(uint32_t left, int right )
{

  return

    ((((int)right) < 0) || (((int)right) >= 32) || (left > ((4294967295U) >> ((int)right)))) ?
    ((left)) :

    (left << ((int)right));
}

static uint32_t
(safe_lshift_func_uint32_t_u_u)(uint32_t left, unsigned int right )
{

  return

    ((((unsigned int)right) >= 32) || (left > ((4294967295U) >> ((unsigned int)right)))) ?
    ((left)) :

    (left << ((unsigned int)right));
}

static uint32_t
(safe_rshift_func_uint32_t_u_s)(uint32_t left, int right )
{

  return

    ((((int)right) < 0) || (((int)right) >= 32)) ?
    ((left)) :

    (left >> ((int)right));
}

static uint32_t
(safe_rshift_func_uint32_t_u_u)(uint32_t left, unsigned int right )
{

  return

    (((unsigned int)right) >= 32) ?
    ((left)) :

    (left >> ((unsigned int)right));
}




static uint64_t
(safe_unary_minus_func_uint64_t_u)(uint64_t ui )
{

  return -ui;
}

static uint64_t
(safe_add_func_uint64_t_u_u)(uint64_t ui1, uint64_t ui2 )
{

  return ui1 + ui2;
}

static uint64_t
(safe_sub_func_uint64_t_u_u)(uint64_t ui1, uint64_t ui2 )
{

  return ui1 - ui2;
}

static uint64_t
(safe_mul_func_uint64_t_u_u)(uint64_t ui1, uint64_t ui2 )
{

  return ((unsigned long long)ui1) * ((unsigned long long)ui2);
}

static uint64_t
(safe_mod_func_uint64_t_u_u)(uint64_t ui1, uint64_t ui2 )
{

  return

    (ui2 == 0) ?
    ((ui1)) :

    (ui1 % ui2);
}

static uint64_t
(safe_div_func_uint64_t_u_u)(uint64_t ui1, uint64_t ui2 )
{

  return

    (ui2 == 0) ?
    ((ui1)) :

    (ui1 / ui2);
}

static uint64_t
(safe_lshift_func_uint64_t_u_s)(uint64_t left, int right )
{

  return

    ((((int)right) < 0) || (((int)right) >= 32) || (left > ((18446744073709551615UL) >> ((int)right)))) ?
    ((left)) :

    (left << ((int)right));
}

static uint64_t
(safe_lshift_func_uint64_t_u_u)(uint64_t left, unsigned int right )
{

  return

    ((((unsigned int)right) >= 32) || (left > ((18446744073709551615UL) >> ((unsigned int)right)))) ?
    ((left)) :

    (left << ((unsigned int)right));
}

static uint64_t
(safe_rshift_func_uint64_t_u_s)(uint64_t left, int right )
{

  return

    ((((int)right) < 0) || (((int)right) >= 32)) ?
    ((left)) :

    (left >> ((int)right));
}

static uint64_t
(safe_rshift_func_uint64_t_u_u)(uint64_t left, unsigned int right )
{

  return

    (((unsigned int)right) >= 32) ?
    ((left)) :

    (left >> ((unsigned int)right));
}
static float
(safe_add_func_float_f_f)(float sf1, float sf2 )
{

  return

    (fabsf((0.5f * sf1) + (0.5f * sf2)) > (0.5f * 3.40282347e+38F)) ?
    (sf1) :

    (sf1 + sf2);
}

static float
(safe_sub_func_float_f_f)(float sf1, float sf2 )
{

  return

    (fabsf((0.5f * sf1) - (0.5f * sf2)) > (0.5f * 3.40282347e+38F)) ?
    (sf1) :

    (sf1 - sf2);
}

static float
(safe_mul_func_float_f_f)(float sf1, float sf2 )
{

  return


    (fabsf((0x1.0p-100f * sf1) * (0x1.0p-28f * sf2)) > (0x1.0p-100f * (0x1.0p-28f * 3.40282347e+38F))) ?



    (sf1) :

    (sf1 * sf2);
}

static float
(safe_div_func_float_f_f)(float sf1, float sf2 )
{

  return


    ((fabsf(sf2) < 1.0f) && (((sf2 == 0.0f) || (fabsf((0x1.0p-49f * sf1) / (0x1.0p100f * sf2))) > (0x1.0p-100f * (0x1.0p-49f * 3.40282347e+38F))))) ?



    (sf1) :

    (sf1 / sf2);
}




static double
(safe_add_func_double_f_f)(double sf1, double sf2 )
{

  return

    (fabs((0.5 * sf1) + (0.5 * sf2)) > (0.5 * 1.7976931348623157e+308)) ?
    (sf1) :

    (sf1 + sf2);
}

static double
(safe_sub_func_double_f_f)(double sf1, double sf2 )
{

  return

    (fabs((0.5 * sf1) - (0.5 * sf2)) > (0.5 * 1.7976931348623157e+308)) ?
    (sf1) :

    (sf1 - sf2);
}

static double
(safe_mul_func_double_f_f)(double sf1, double sf2 )
{

  return


    (fabs((0x1.0p-100 * sf1) * (0x1.0p-924 * sf2)) > (0x1.0p-100 * (0x1.0p-924 * 1.7976931348623157e+308))) ?



    (sf1) :

    (sf1 * sf2);
}

static double
(safe_div_func_double_f_f)(double sf1, double sf2 )
{

  return


    ((fabs(sf2) < 1.0) && (((sf2 == 0.0) || (fabs((0x1.0p-974 * sf1) / (0x1.0p100 * sf2))) > (0x1.0p-100 * (0x1.0p-974 * 1.7976931348623157e+308))))) ?



    (sf1) :

    (sf1 / sf2);
}
static int32_t
(safe_convert_func_float_to_int32_t)(float sf1 )
{

  return

    ((sf1 <= (-2147483647-1)) || (sf1 >= (2147483647))) ?
    ((2147483647)) :

    ((int32_t)(sf1));
}

static uint32_t crc32_tab[256];
static uint32_t crc32_context = 0xFFFFFFFFUL;

static void
crc32_gentab (void)
{
 uint32_t crc;
 const uint32_t poly = 0xEDB88320UL;
 int i, j;

                            
          
                           
                 
                            
           
              
    
   
                     
  
}

static void
crc32_byte (uint8_t b) {
                
                                       
                                        
}
static void
crc32_8bytes (uint64_t val)
{
                              
                              
                               
                               
                               
                               
                               
                               
}

static void
transparent_crc (uint64_t val, char* vname, int flag)
{
                   
            
                                                                                        
  
}



static void
transparent_crc_bytes (char *ptr, int nbytes, char* vname, int flag)
{
    int i;
                              
                           
     
            
                                                                                        
  
}
static long __undefined;

struct S0 {
   signed f0 : 1;
   int64_t f1;
   signed f2 : 20;
   unsigned f3 : 8;
};

static int32_t g_2 = 1L;
static struct S0 g_35 = {-0,-1L,631,9};
static uint16_t g_40 = 65534UL;
static int32_t g_85 = 0x89AB98CFL;
static uint8_t g_86 = 2UL;
static int64_t g_131 = 1L;
static int16_t g_153 = 0L;
static uint32_t g_154 = 3UL;
static int16_t g_158 = 0L;
static uint32_t g_159 = 0x5CC1FFD3L;
static uint32_t g_203 = 0xBD2EE514L;
static int16_t g_250 = 0x8C40L;
static uint32_t g_251 = 0xB89A725EL;
static uint8_t g_312 = 1UL;
static int32_t g_316 = 0x123013CDL;
static uint32_t g_355 = 9UL;
static int32_t g_356 = (-1L);
static struct S0 g_388 = {0,6L,-661,1};
static int32_t g_390 = (-1L);
static uint8_t g_391 = 255UL;
static uint16_t g_420[4][10] = {{65535UL,0x88F0L,65535UL,65532UL,0x88F0L,7UL,7UL,0x88F0L,65532UL,65535UL},{0UL,0UL,0x54A8L,0x88F0L,0x8AC6L,0x54A8L,0x8AC6L,0x88F0L,0x54A8L,0UL},{0x8AC6L,7UL,65535UL,0x8AC6L,65532UL,65532UL,0x8AC6L,65535UL,7UL,0x8AC6L},{65535UL,0UL,7UL,65532UL,0UL,65532UL,7UL,0UL,65535UL,65535UL}};
static int32_t g_421[5][10][5] = {{{(-1L),0x9CC7936FL,0x733E4B69L,(-10L),3L},{0x6F17C7B4L,0x504E397CL,0xFAF5C5A5L,0x7C3E73E6L,(-1L)},{9L,0x276B277BL,(-1L),6L,6L},{(-8L),0xA1E38021L,(-8L),0x94C3BD62L,0xA7408F1FL},{0x8AC3A7DBL,0xD1F34A63L,0x05469B73L,0x41E353BDL,0x2626DBABL},{0x6F17C7B4L,0L,(-3L),1L,0xC693E4D5L},{(-1L),0x468DFB35L,0x05469B73L,0x2626DBABL,3L},{0xD2A2360EL,(-9L),(-8L),0x962C5FE5L,0x6F17C7B4L},{0x44593456L,0x9CC7936FL,(-1L),0xADF75AD1L,0xD7FDC368L},{0xC7DFE44CL,0x889A9044L,0xFAF5C5A5L,(-1L),0x504E397CL}},{{0xD1F34A63L,0x7D366DB7L,0xFA444388L,0x468DFB35L,0x4A120EAAL},{0x7C3E73E6L,0x40DD39B6L,0xD06CBE39L,0x73BCDCAEL,0x92D6179AL},{3L,0L,0xADF75AD1L,0L,0x8AC3A7DBL},{0x889A9044L,7L,0x2DC81D43L,0xB3515830L,0xFAF5C5A5L},{0xFA444388L,0x05469B73L,1L,(-5L),1L},{0x34421670L,0x34421670L,(-1L),(-1L),0x73BCDCAEL},{0xB2F5332AL,0x2626DBABL,0x7E2228E0L,0x28B7BCCEL,0x276B277BL},{0xDE5C95E5L,0L,(-4L),0xA3EA5012L,0xDA0AB8EFL},{0xD5DD3E96L,0x2626DBABL,9L,0x13025689L,1L},{0L,0x34421670L,0x57F46E1EL,0x40DD39B6L,0xC7DFE44CL}},{{0x28B7BCCEL,0x05469B73L,1L,1L,0x7D366DB7L},{(-1L),7L,0x962C5FE5L,0L,0xDFEFCE86L},{0x0B56072FL,0L,0x733E4B69L,0xD7FDC368L,0L},{(-1L),0x40DD39B6L,(-1L),0xFAF5C5A5L,0xD2A2360EL},{0xCD05DC4EL,0x7D366DB7L,1L,0x4772FBBAL,0xBB4B3D1CL},{0xDA0AB8EFL,0x889A9044L,0xA3EA5012L,0x6F17C7B4L,(-1L)},{(-1L),0x9CC7936FL,0xB2F5332AL,0xB2F5332AL,0x9CC7936FL},{0xC693E4D5L,(-9L),0x504E397CL,0xD06CBE39L,0xA3EA5012L},{1L,0x468DFB35L,9L,(-7L),0x2871C56AL},{0xD06CBE39L,0L,0xEB725935L,(-1L),1L}},{{1L,0xD1F34A63L,(-1L),1L,0x13025689L},{0xC693E4D5L,0xA1E38021L,1L,(-1L),0x5D2BDD0BL},{(-1L),0x276B277BL,0x2871C56AL,(-1L),6L},{0xDA0AB8EFL,0x504E397CL,0x5D2BDD0BL,7L,0L},{0xCD05DC4EL,(-1L),0xBB4B3D1CL,3L,0L},{(-1L),(-1L),8L,0x3E7F1E87L,0x962C5FE5L},{0x0B56072FL,0x28B7BCCEL,0L,0x2871C56AL,(-1L)},{(-1L),0x0552E894L,(-1L),0x5CF14D71L,(-8L)},{0x28B7BCCEL,(-1L),8L,(-1L),0x28B7BCCEL},{0L,(-8L),0xA7408F1FL,0L,1L}},{{(-1L),0x2871C56AL,0L,0x28B7BCCEL,0x0B56072FL},{1L,1L,(-1L),0xB7C597CEL,1L},{6L,0x28B7BCCEL,0xD1F34A63L,0x9CC7936FL,3L},{1L,0x45CBE6D6L,(-3L),0x3E7F1E87L,0xB7C597CEL},{0xBC0FE9D4L,1L,0x4772FBBAL,1L,3L},{0x40DD39B6L,0xC693E4D5L,0x0552E894L,0x6F17C7B4L,0L},{0xCD05DC4EL,(-5L),0x05469B73L,(-1L),0x7E2228E0L},{0L,0x0552E894L,1L,1L,0xFAF5C5A5L},{0xBB4B3D1CL,0x276B277BL,9L,1L,0L},{0x3E7F1E87L,0L,0x962C5FE5L,(-1L),0xD2A2360EL}}};
static uint64_t g_506 = 0UL;
static uint64_t g_559[8] = {4UL,4UL,4UL,4UL,4UL,4UL,4UL,4UL};
static int16_t g_594 = 1L;
static uint16_t g_750[1][6][1] = {{{0xE455L},{0xE455L},{0xE455L},{0xE455L},{0xE455L},{0xE455L}}};
static uint64_t g_754[6][10] = {{0x91FF780DDFEFE0BALL,0x91FF780DDFEFE0BALL,0xB71EF27D511E6B97LL,0x79AB76C8654775D1LL,0xF443B1863DBA0E6ALL,18446744073709551608UL,0x91FF780DDFEFE0BALL,0xF443B1863DBA0E6ALL,0UL,0xF443B1863DBA0E6ALL},{0x91FF780DDFEFE0BALL,0x5376858CA911C115LL,0x7F1EB7A26DC46069LL,0x79AB76C8654775D1LL,0x7F1EB7A26DC46069LL,0x5376858CA911C115LL,0x91FF780DDFEFE0BALL,0x7F1EB7A26DC46069LL,18446744073709551615UL,0xF443B1863DBA0E6ALL},{0x5376858CA911C115LL,0x91FF780DDFEFE0BALL,0x7F1EB7A26DC46069LL,18446744073709551615UL,0xF443B1863DBA0E6ALL,0x5376858CA911C115LL,0x5376858CA911C115LL,0xF443B1863DBA0E6ALL,18446744073709551615UL,0x7F1EB7A26DC46069LL},{0x91FF780DDFEFE0BALL,0x91FF780DDFEFE0BALL,0xB71EF27D511E6B97LL,0x79AB76C8654775D1LL,0xF443B1863DBA0E6ALL,18446744073709551608UL,0x91FF780DDFEFE0BALL,0xF443B1863DBA0E6ALL,0UL,0xF443B1863DBA0E6ALL},{0x91FF780DDFEFE0BALL,0x5376858CA911C115LL,0x7F1EB7A26DC46069LL,0x79AB76C8654775D1LL,0x7F1EB7A26DC46069LL,0x5376858CA911C115LL,0x91FF780DDFEFE0BALL,0x7F1EB7A26DC46069LL,18446744073709551615UL,0xF443B1863DBA0E6ALL},{0x5376858CA911C115LL,0x91FF780DDFEFE0BALL,0x7F1EB7A26DC46069LL,18446744073709551615UL,0xF443B1863DBA0E6ALL,0x5376858CA911C115LL,0x5376858CA911C115LL,0xF443B1863DBA0E6ALL,18446744073709551615UL,0x7F1EB7A26DC46069LL}};
static int32_t g_755[8] = {0x7375C0FDL,0x7375C0FDL,0x5F05B9D7L,0x7375C0FDL,0x7375C0FDL,0x5F05B9D7L,0x7375C0FDL,0x7375C0FDL};
static int32_t g_1021[5] = {1L,1L,1L,1L,1L};
static int32_t g_1031 = 1L;
static uint32_t g_1032[9] = {18446744073709551615UL,18446744073709551606UL,18446744073709551615UL,18446744073709551606UL,18446744073709551615UL,18446744073709551606UL,18446744073709551615UL,18446744073709551606UL,18446744073709551615UL};
static int32_t g_1103 = 0x44377EFEL;
static int8_t g_1109 = (-10L);
static struct S0 g_1145[8] = {{-0,-1L,887,10},{-0,-1L,887,10},{-0,-1L,887,10},{-0,-1L,887,10},{-0,-1L,887,10},{-0,-1L,887,10},{-0,-1L,887,10},{-0,-1L,887,10}};
static int64_t g_1238 = 0xEF73CDC07DB9FD50LL;
static uint64_t g_1241 = 1UL;
static uint32_t g_1304 = 0UL;
static uint32_t g_1732[2][7] = {{4294967287UL,4294967287UL,4294967287UL,4294967287UL,4294967287UL,4294967287UL,4294967287UL},{0xA22DCD37L,0UL,0xA22DCD37L,0UL,0xA22DCD37L,0UL,0xA22DCD37L}};
static uint64_t g_1756[1][1][10] = {{{1UL,2UL,1UL,1UL,2UL,1UL,1UL,2UL,1UL,1UL}}};
static uint32_t g_1935 = 3UL;
static int64_t g_1985 = 0x4E20E02FB14D3ADFLL;
static uint32_t g_2138 = 0xE3D39B8FL;
static uint8_t g_2239 = 0x2BL;
static uint32_t g_2299 = 0x533C3544L;
static int32_t g_2300 = (-1L);
static int32_t g_2342 = 0x91C31F8BL;

static int64_t func_1(void);
static int32_t func_6(uint32_t p_7, uint32_t p_8, int64_t p_9, int8_t p_10, int32_t p_11);
static uint8_t func_12(uint32_t p_13, int16_t p_14, struct S0 p_15, struct S0 p_16, int8_t p_17);
static uint8_t func_22(uint32_t p_23);
static uint8_t func_41(int32_t p_42, uint8_t p_43, int32_t p_44, int64_t p_45, uint32_t p_46);
static uint16_t func_51(int16_t p_52, uint16_t p_53, uint64_t p_54, int32_t p_55);
static struct S0 func_59(uint32_t p_60, int16_t p_61, uint64_t p_62, uint8_t p_63, struct S0 p_64);
static int32_t func_66(uint16_t p_67, struct S0 p_68);
static int32_t func_71(uint8_t p_72, uint32_t p_73, uint32_t p_74);
static uint8_t func_78(uint32_t p_79);






static int64_t func_1(void)
{
    uint32_t l_5 = 0x6DA5043CL;
    struct S0 l_34 = {0,0x5C82F744ED25E425LL,-208,14};
    int32_t l_2303[5];
    int8_t l_2308 = 3L;
    uint16_t l_2311 = 65527UL;
    uint16_t l_2345[4] = {0xEDBEL,0xEDBEL,0xEDBEL,0xEDBEL};
    uint32_t l_2354 = 0x7FA494A4L;
    int32_t l_2357 = 0x75FE5BB8L;
    uint16_t l_2358 = 0UL;
    int i;
                           
                          
    for (g_2 = 22; (g_2 <= (-3)); g_2 = safe_sub_func_int16_t_s_s(g_2, 2))
    {
        uint32_t l_30 = 0UL;
        uint8_t l_33 = 255UL;
        int32_t l_2301 = 0x444D8480L;
        int32_t l_2302 = (-2L);
        int32_t l_2304 = 0x33EA3733L;
        int32_t l_2305 = 0x25C00CA3L;
        int32_t l_2306 = 0L;
        int32_t l_2307 = 7L;
        int32_t l_2309 = 3L;
        int32_t l_2310[1];
        int16_t l_2343 = (-1L);
        uint32_t l_2344 = 1UL;
        int i;






{
        uint32_t l_30 = 0UL;
        uint8_t l_33 = 255UL;
        int32_t l_2301 = 0x444D8480L;
        int32_t l_2302 = (-2L);
        int32_t l_2304 = 0x33EA3733L;
        int32_t l_2305 = 0x25C00CA3L;
        int32_t l_2306 = 0L;
        int32_t l_2307 = 7L;
        int32_t l_2309 = 3L;
        int32_t l_2310[1];
        int16_t l_2343 = (-1L);
        uint32_t l_2344 = 1UL;
        int i;
                               
                                    
                  
        g_2300 |= func_6(((18446744073709551615UL != 0x315362DAAB72B21ALL) > func_12((g_2 == (safe_rshift_func_uint16_t_u_u((safe_sub_func_int64_t_s_s(0x4CCFD66C4A87FCE6LL, (g_2 && func_22((safe_sub_func_uint16_t_u_u((safe_add_func_uint64_t_u_u((0UL && ((safe_mul_func_int8_t_s_s(l_30, (safe_sub_func_int64_t_s_s((4294967290UL != ((l_33 ^ g_2) > l_30)), g_2)))) & 65534UL)), 0x2AD015B01D976D99LL)), g_2)))))), l_5))), l_5, l_34, g_35, l_34.f0)), l_30, l_33, g_2, l_33);
                 
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  
    }




                                                                       




                  



          
                             

                              

 



                                      




 
                                            
                     

            
           




                                      
                                                                                    
                                       
                                                                                                                                     




                                                  




                                           


    }
                                                                                                                                                                                                                                                                                                                                                         
             
             
    return g_1756[0][0][6];
}





static int32_t func_6(uint32_t p_7, uint32_t p_8, int64_t p_9, int8_t p_10, int32_t p_11)
{
    return p_8;
}





static uint8_t func_12(uint32_t p_13, int16_t p_14, struct S0 p_15, struct S0 p_16, int8_t p_17)
{
    uint16_t l_47[9][1];
    int32_t l_48 = 0x693CE85DL;
    int32_t l_58 = 1L;
    struct S0 l_65 = {-0,0x398C3D34E0BB6923LL,83,9};
    int32_t l_2073 = 1L;
    int32_t l_2076 = 0xF76A42B0L;
    int32_t l_2077 = 0x8D8B355DL;
    int32_t l_2078 = 1L;
    int32_t l_2080[3][2][9];
    int32_t l_2166 = 0x1B7F9CF5L;
    int8_t l_2167 = 1L;
    uint16_t l_2242[1];
    int32_t l_2280 = 0xA36C0F69L;
    struct S0 l_2282[6] = {{0,-1L,256,10},{0,-1L,256,10},{0,-1L,256,10},{0,-1L,256,10},{0,-1L,256,10},{0,-1L,256,10}};
    uint32_t l_2298 = 0xB81F1C22L;
    int i, j, k;

                                       


                                                    
                                                                                                                                                                                                                                                                                                                                                                                                                                              
                                                  




                            







                    



                                                                                     



                       



           




          
 
                                    
                                                                             
                     

                  


 
                               
         
                                   
                                     
         
     

                                              
                                          





                                                   


                              




           


                       
     


            


     ;
                                      



                                    





                                         






          




                                                                                                    





           
                                         




                                          




safe_mul_func_int16_t_s_s((func_51(((safe_lshift_func_int8_t_s_s(l_58, 3)) , 1L), (func_22(((g_1103 |= (func_59((p_15.f3 ^= 0x43772679L), p_16.f3, p_15.f2, g_35.f2, l_65) , p_15.f2)) || g_1021[4])) , g_1021[3]), l_65.f1, g_1021[3]) && 0L), g_1021[3]);




              


 


                       

           
                                                                                                                                                                                                                           



           
                                                         


             


                                                         
                                                                                                                    


                                       
                                        


 
                                                         
                 


                                       
                 
             


                                          


                                       


                                                     
                                                                                                                                                                                                                           






                                               


         

                



                                                                                                    







                                                         




                                    




                



                       
         

 

                                                                       



 
                              
                                     
                                               
                          
                                     
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          
                            
                              
                    
                               
             

                                                                                                                                                          

 



                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               
     
                                     
           




                                            




                                          




                                      






                                                                                         
                                            


    return g_2299;
}





static uint8_t func_22(uint32_t p_23)
{
    return p_23;
}





static uint8_t func_41(int32_t p_42, uint8_t p_43, int32_t p_44, int64_t p_45, uint32_t p_46)
{
    uint16_t l_1480 = 0xA686L;
    int32_t l_1505 = 0L;
    int32_t l_1506 = (-9L);
    int16_t l_1507 = 0L;
    int32_t l_1508 = 0L;
    struct S0 l_1570[3][4][5] = {{{{0,-4L,-116,7},{0,0x5F448C1F958DC280LL,-423,15},{0,8L,-490,8},{0,8L,-490,8},{0,0x5F448C1F958DC280LL,-423,15}},{{0,0xA638DFE240EFA1CBLL,-152,11},{0,0x919B657A6F0D3D43LL,-795,11},{-0,0x603356D2FC8A99DCLL,-575,2},{0,-1L,825,10},{0,-1L,825,10}},{{0,0xA0191CBE08CA11F3LL,-235,10},{-0,0L,386,13},{0,0xA0191CBE08CA11F3LL,-235,10},{0,8L,-490,8},{0,1L,900,6}},{{0,0L,88,3},{-0,0xD3427C9C5EA38133LL,50,9},{0,-1L,825,10},{-0,0xD3427C9C5EA38133LL,50,9},{0,0L,88,3}}},{{{0,0xA0191CBE08CA11F3LL,-235,10},{0,-4L,-116,7},{-0,0L,386,13},{0,0x5F448C1F958DC280LL,-423,15},{-0,0L,386,13}},{{0,0xA638DFE240EFA1CBLL,-152,11},{0,0xA638DFE240EFA1CBLL,-152,11},{0,-1L,825,10},{0,0L,88,3},{0,0xEAA05D5F758D6168LL,987,14}},{{0,-4L,-116,7},{0,0xA0191CBE08CA11F3LL,-235,10},{0,0xA0191CBE08CA11F3LL,-235,10},{0,-4L,-116,7},{-0,0L,386,13}},{{-0,0xD3427C9C5EA38133LL,50,9},{0,0L,88,3},{-0,0x603356D2FC8A99DCLL,-575,2},{-0,0x603356D2FC8A99DCLL,-575,2},{0,0L,88,3}}},{{{-0,0L,386,13},{0,0xA0191CBE08CA11F3LL,-235,10},{0,8L,-490,8},{0,1L,900,6},{0,1L,900,6}},{{0,0x919B657A6F0D3D43LL,-795,11},{0,0xA638DFE240EFA1CBLL,-152,11},{0,0x919B657A6F0D3D43LL,-795,11},{-0,0x603356D2FC8A99DCLL,-575,2},{0,-1L,825,10}},{{0,0x5F448C1F958DC280LL,-423,15},{0,-4L,-116,7},{0,1L,900,6},{0,-4L,-116,7},{0,0x5F448C1F958DC280LL,-423,15}},{{0,0x919B657A6F0D3D43LL,-795,11},{-0,0xD3427C9C5EA38133LL,50,9},{0,0xA638DFE240EFA1CBLL,-152,11},{0,0L,88,3},{0,0xA638DFE240EFA1CBLL,-152,11}}}};
    int32_t l_1694 = 0x478A7D77L;
    int32_t l_1695 = (-3L);
    int64_t l_1700 = (-2L);
    int32_t l_1701 = 1L;
    int32_t l_1702 = 0x753BEF4DL;
    int32_t l_1878 = 1L;
    uint8_t l_1943[8] = {0xFCL,0xFCL,0xFCL,0xFCL,0xFCL,0xFCL,0xFCL,0xFCL};
    int i, j, k;
         
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            
     
                                     
                                         
                                                  
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  
     
        
     
                                      
                                       
                                          
                                     
                                                                                                                                                                                                                                                                                                                                                 
                 
                                                                                   
         
                                              
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   
             
                                       
                                       
                                                 
                                        
                                                      
                 
                                          
                                                 
                                                             
                     
                                                     
                                                                                 
                                 
                                                     
                                  
                                 
                                   
                     
                                                                
                     
                                                     
                              
                                                 
                                                    
                                                                                                                                                                                                                                                                                                                                                                                                                                             
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      
                     
                                                                                               
                     
                                                      
                                              
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               
                     
                 
             
                
             
                         
                          
                             
             
         
     
         
                    


                                                                                                                                                                                                           




                                              




 




                                       
                 


           


                                              
                                   
                                            
                                                
    return p_42;
}





static uint16_t func_51(int16_t p_52, uint16_t p_53, uint64_t p_54, int32_t p_55)
{
    uint8_t l_1104[4][4][5] = {{{0x67L,0xA6L,0x5DL,0x66L,0UL},{0xA6L,0xAEL,0x21L,251UL,4UL},{0x67L,255UL,9UL,0xAEL,254UL},{0xC1L,1UL,1UL,0x5DL,255UL}},{{1UL,253UL,0xA6L,0UL,0x4FL},{253UL,253UL,255UL,0x33L,0x33L},{255UL,1UL,255UL,0x7DL,255UL},{255UL,255UL,253UL,0x21L,1UL}},{{251UL,0xAEL,0x12L,255UL,0xC1L},{255UL,0xA6L,253UL,1UL,0xC1L},{0x41L,7UL,255UL,0x04L,253UL},{9UL,253UL,255UL,254UL,0x6CL}},{{7UL,255UL,0xA6L,254UL,0x67L},{0x21L,0x04L,1UL,0x04L,0x21L},{0x12L,0x21L,9UL,1UL,255UL},{0x5DL,253UL,0x21L,255UL,0xAEL}}};
    struct S0 l_1144 = {0,0x2C0EE27351D2F17ALL,-548,3};
    int32_t l_1162 = 0x0D39437AL;
    int32_t l_1354 = 1L;
    int32_t l_1356[4] = {(-5L),(-5L),(-5L),(-5L)};
    uint16_t l_1363 = 0xE5EBL;
    uint16_t l_1426[2];
    struct S0 l_1427 = {0,-1L,929,12};
    int i, j, k;

                                     






            


                                                                            
     
                                      
                                     

                                                      



                                        


                                                                   
                                                                   




          


                 
                                                                                                    


 
                                           
                                              


                 




                                              


     ;
          


                                                    
                                    
                                                                                                    




{
        uint32_t l_1440 = 0x3482DA34L;
        int32_t l_1452 = 0xB168BE91L;
                                                 
         
                                                                                            
             
                                                                                      
                 
                                                  
                                  
                 
             
         
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    
        p_55 = (((l_1427.f2 = p_53) & (l_1452 &= (g_35.f1 , 0x2C6BL))) , ((safe_rshift_func_int8_t_s_u(((safe_rshift_func_uint16_t_u_s((safe_add_func_int16_t_s_s((~((l_1452 = 0xE489E68586BFF0DDLL) ^ l_1440)), p_53)), (safe_mod_func_int8_t_s_s((0xAA7170BFL && 0x73A17C9BL), (safe_add_func_int32_t_s_s((0xAFL != 0x3BL), l_1440)))))) != p_52), 4)) & p_52));
    }


                                                         


                                        
                                                                            
     
                                      
                                     
                                                 
         
                                                                                            
             
                                                                                      
                 
                                                  
                                  
                 
             
         
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    
                                                                                                                                                                                                                                                                                                                                                                  
     ;



                                          



                                  
                                                           




                                              




 
                                                                                            
             
                                                                                      
                 
                                                  
                                  
                 
             
         

                                              
                                          
 
                                           
                 

                                              


          


                                                                            
     
                                      
                                     
                                                 
         


                                          

         
                                         




                                                                                                                                                                                                                                                                                                                                                                  
                                       


     ;
          
            
                                                                                                                                                                                                                       




                                                                 




                                                                                                                                                                                                                                                                                      




safe_lshift_func_int8_t_s_s(2L, 2);




                                   




          
                                          
     
                                          
                                   
     ;


                                       
             


                    
          



                                              




                                          
     
                                          
                                   
     ;




 
                                           
                     


                                                          





            




 
                               
                                                 
                                          
                                     
                                     
                           
                                     
              


                                             




                                          
     
                                          
                                   
      


                                  
          




            




                                          




              


                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            
                                                                                                                                                                                            


     
 



                                              


                                                                                                                                     
                                                                                                                                                                                                                       


             
{
        int32_t l_1164 = (-2L);
        uint32_t l_1176 = 18446744073709551615UL;
        struct S0 l_1240 = {-0,0L,291,15};
        int32_t l_1360 = 0x5D9D81B5L;
        int32_t l_1362 = 0xBA92A2E7L;
        uint16_t l_1423[2];
        int32_t l_1428 = 0xF2E968EEL;
        int i;


                                       


                






                                                                                                                                                                                                                                




                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    

 


                                          
     
                                          
                                   
      


               
                                           




               



                                        


             


          




                                      




                                            


func_59((l_1164 = 4294967289UL), g_86, p_55, (((~((((safe_sub_func_uint16_t_u_u(((g_1021[3] , (((((safe_mul_func_int8_t_s_s((g_312 || ((((safe_lshift_func_int8_t_s_u((safe_mul_func_int16_t_s_s((safe_add_func_int32_t_s_s((((l_1144.f2 = ((++g_506) , (safe_rshift_func_int16_t_s_s((0x88DAE7C37C3C3BB9LL != (((l_1240.f2 , (l_1423[0] = l_1144.f3)) >= ((((safe_mod_func_int16_t_s_s(8L, 0x9ED3L)) , 0x7B50B71F29030D9BLL) > g_420[2][1]) && g_85)) , (-7L))), g_1145[0].f0)))) <= l_1360) && 0x89L), g_131)), p_52)), l_1144.f0)) < g_355) , l_1104[3][3][4]) > l_1363)), p_53)) <= g_390) , p_54) & l_1426[0]) || g_2)) & g_594), g_391)) <= g_85) , l_1240.f0) < l_1362)) && g_390) , g_421[3][7][1]), l_1427);
                                           



                                                                     


                                   
 
                                                                                      
                 
                                                  
                                  
                 
             


                                                                                     






                                    


{
        int32_t l_1164 = (-2L);
        uint32_t l_1176 = 18446744073709551615UL;
        struct S0 l_1240 = {-0,0L,291,15};
        int32_t l_1360 = 0x5D9D81B5L;
        int32_t l_1362 = 0xBA92A2E7L;
        uint16_t l_1423[2];
        int32_t l_1428 = 0xF2E968EEL;
        int i;


                                  
                                    




for (g_391 = (-7); (g_391 != 54); g_391++)
    {
        struct S0 l_1479 = {0,-1L,935,13};
        l_1427 = (l_1479 = l_1427);
    };


                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            
    }



 
                                           
                 






 
                                                  
                                  
                 
    }
    return g_755[7];
}





static struct S0 func_59(uint32_t p_60, int16_t p_61, uint64_t p_62, uint8_t p_63, struct S0 p_64)
{
    int16_t l_75 = (-1L);
    int32_t l_80 = 0xDA297315L;
    int32_t l_81 = 0xEAE3254CL;
    int32_t l_82 = 0x588CB35BL;
    int64_t l_83 = (-5L);
    int32_t l_84[10][9] = {{0xA6B6B07CL,0xA9E7339AL,(-1L),0xA9E7339AL,0xA6B6B07CL,0xA9E7339AL,(-1L),0xA9E7339AL,0xA6B6B07CL},{0xF4112691L,0xAA3CE9D1L,0xD0B4E555L,0x43A44ADEL,0x43A44ADEL,0xD0B4E555L,0xAA3CE9D1L,0xF4112691L,0xF4112691L},{0xC018302AL,0xA9E7339AL,0xC018302AL,0x01D2D5FAL,0xC018302AL,0xA9E7339AL,0xC018302AL,0x01D2D5FAL,0xC018302AL},{0xF4112691L,0x43A44ADEL,0xAA3CE9D1L,0xAA3CE9D1L,0x43A44ADEL,0xF4112691L,0xD0B4E555L,0xD0B4E555L,0xF4112691L},{0xA6B6B07CL,0x01D2D5FAL,(-1L),0x01D2D5FAL,0xA6B6B07CL,0x01D2D5FAL,(-1L),0x01D2D5FAL,0xA6B6B07CL},{0x43A44ADEL,0xAA3CE9D1L,0xAA3CE9D1L,0x43A44ADEL,0xF4112691L,0xD0B4E555L,0xD0B4E555L,0xF4112691L,0x43A44ADEL},{0xC018302AL,0x01D2D5FAL,0xC018302AL,0xA9E7339AL,0xC018302AL,0x01D2D5FAL,0xC018302AL,0xA9E7339AL,0xC018302AL},{0x43A44ADEL,0x43A44ADEL,0xD0B4E555L,0xAA3CE9D1L,0xF4112691L,0xF4112691L,0xAA3CE9D1L,0xD0B4E555L,0x43A44ADEL},{0xA6B6B07CL,0xA9E7339AL,(-1L),0xA9E7339AL,0xA6B6B07CL,0xA9E7339AL,(-1L),0xA9E7339AL,0xA6B6B07CL},{0xF4112691L,0xAA3CE9D1L,0xD0B4E555L,0x43A44ADEL,0x43A44ADEL,0xD0B4E555L,0xAA3CE9D1L,0xF4112691L,0xF4112691L}};
    struct S0 l_163 = {0,0x74F8CE80C63ED81ALL,-372,13};
    int64_t l_603[2];
    uint64_t l_606 = 0UL;
    uint64_t l_712 = 0xDB55FDE756C50D62LL;
    uint32_t l_716 = 1UL;
    int32_t l_902 = 0x0DED8928L;
    int64_t l_959[4];
    int32_t l_961 = 0L;
    int i, j;

                                                         




                                                                                     






 


     






                                                                              

           



                                                                                                  




                                   




                                    


                       

                                               




                                                                   




                                                                                                  




lbl_903:
                                              
     
                                
                                    
                  
                  
                 
                  
                                                                                                                                                                                                                                                                        
                
     


                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     


                                    

                                             







          
                             
                                        
                                      


                                                                                     


           


                                      


          
                goto lbl_903;



                                                                                                                                                                                                                                                                                                                                                                  



                                                                     


    return p_64;
}





static int32_t func_66(uint16_t p_67, struct S0 p_68)
{
    uint32_t l_168 = 0x3F8D3DCDL;
    int8_t l_179[1][2];
    int64_t l_185 = (-7L);
    int32_t l_228 = 0x185051ADL;
    int32_t l_230 = 0x647FE7AAL;
    uint32_t l_268 = 0x20F4CF03L;
    int16_t l_359 = (-6L);
    int32_t l_473 = 0xDCCA3664L;
    int32_t l_510 = 0L;
    int32_t l_512[1][3][8];
    uint16_t l_544 = 6UL;
    int64_t l_551 = 6L;
    int32_t l_560 = 0x0C92ECC8L;
    struct S0 l_567[2][6][5] = {{{{-0,0x71786727EA7006D1LL,552,9},{0,0x42C4DEBBA0133BA3LL,-707,0},{0,0x8519C44ADFC325C0LL,30,11},{0,0L,262,3},{-0,0L,-471,1}},{{-0,-10L,122,9},{-0,0x71786727EA7006D1LL,552,9},{0,5L,628,13},{-0,0L,-471,1},{-0,0xC86EFBAF48BD29F1LL,-736,6}},{{0,1L,478,4},{0,0x8519C44ADFC325C0LL,30,11},{-0,0L,-471,1},{0,0x42C4DEBBA0133BA3LL,-707,0},{-0,0L,-471,1}},{{-0,0L,-471,1},{-0,0L,-471,1},{-0,-1L,-604,9},{0,5L,628,13},{0,-1L,-35,12}},{{-0,0L,-471,1},{-0,7L,693,13},{0,-1L,-35,12},{0,0xD0D1A3F0EE69D5CDLL,-352,14},{0,-3L,-677,2}},{{0,1L,478,4},{0,0x7C6C453A6D93444DLL,535,5},{-0,0xC86EFBAF48BD29F1LL,-736,6},{-0,0L,-902,5},{0,0L,262,3}}},{{{-0,-10L,122,9},{-0,7L,693,13},{-0,7L,693,13},{-0,-10L,122,9},{-0,1L,344,2}},{{-0,0x71786727EA7006D1LL,552,9},{-0,0L,-471,1},{-0,7L,693,13},{0,-1L,-35,12},{0,0xD0D1A3F0EE69D5CDLL,-352,14}},{{0,-1L,-35,12},{0,0x8519C44ADFC325C0LL,30,11},{-0,0xC86EFBAF48BD29F1LL,-736,6},{0,1L,478,4},{-0,7L,693,13}},{{0,0x7C6C453A6D93444DLL,535,5},{-0,0x71786727EA7006D1LL,552,9},{0,-1L,-35,12},{0,-1L,-35,12},{-0,0x71786727EA7006D1LL,552,9}},{{0,-3L,-677,2},{0,0x42C4DEBBA0133BA3LL,-707,0},{-0,-1L,-604,9},{-0,-10L,122,9},{-0,0x71786727EA7006D1LL,552,9}},{{0,0x8519C44ADFC325C0LL,30,11},{0,0L,262,3},{-0,0L,-471,1},{-0,0L,-902,5},{-0,7L,693,13}}}};
    int i, j, k;

                                        


                                          
                                   






                                                                                                     






 


                     


                                                                                                                                     




                                                                           



                       
         
                                   
                                             
         ;


                                           
                                                                                                     





                                                                                     
                              
                 


                                       
                                       


                                                                                                       


                                                                           




                                       
                                      


                                             
                                   



                                        


           




              




                                                                 






 


                     







           
                                        







                                                                 



                                       



                            


                                   




           


 


                                                                     


                                    
                       
     
                               
                                
      




                                   



                                                               


                                                                       
                                          


 
                                                                                                                                                                                                                                                                                                         
                     

                 


                  


                                                                                                                            

                            


                                                                                                       

          




                       
                                             


            






                                         


                                           
                                     




                                       




    return p_67;
}





static int32_t func_71(uint8_t p_72, uint32_t p_73, uint32_t p_74)
{
    int32_t l_145 = 1L;
    int32_t l_148[4];
    int32_t l_151[6] = {(-1L),0x6C71F3ECL,0x6C71F3ECL,(-1L),0x6C71F3ECL,0x6C71F3ECL};
    int32_t l_152 = 0xF6C6AD03L;
    int32_t l_155 = 3L;
    int8_t l_162 = 0x2DL;
    int i;







 

                                                                     
                       

     

                                      


 


               

                     
           
              


                                                       




                              


                                                   

                                             




                                                                     




              



           



                                                                                    



 
                                                                           
     

                                          


                                                                                                       


                                    


                                        

            



                                              




           




           






          


                                         
                       
         


                                                                                                                                                                                                                                                                                                                                                                                                                                     
                   




 
           


                                                                                                                                                                                                                                                                                                                                                                                                                                                         


          




 


                    

                     
     


                                    
                                                   
                                                                                                                                                                                                                                                           


                                                                                                                                                                                                             


                                             




                                                                   




                                                                              


                                                                                                                                     
                                                                                                                                                                                                                                                                                                                                                                                                  
    return g_85;
}





static uint8_t func_78(uint32_t p_79)
{
    int8_t l_93 = 0xFCL;
    uint32_t l_94 = 0xE62B9925L;
    uint8_t l_95 = 1UL;
    int32_t l_96 = 0x683588D2L;
    int32_t l_105 = 0x05D52AEBL;
    int64_t l_124[9] = {0x184C74488338C196LL,0x184C74488338C196LL,0x184C74488338C196LL,0x184C74488338C196LL,0x184C74488338C196LL,0x184C74488338C196LL,0x184C74488338C196LL,0x184C74488338C196LL,0x184C74488338C196LL};
    int i;


                                                       


                                         
                 


                                       
                 ;



                                          



                                                                                                 


                                                                
     
                    
     
        
     
                           
     ;
                                                                                                                                     




                                           




                                            






                                        


           
          
                                                   

           




                                                                   





                                       


 


 
                                            
                     

                 

                                       


 

                                                                                                       


     

                                       




                                                       


                                      


    return g_40;
}

int main (void)
{
    int i, j, k;
    int print_hash_value = 0;
                   
    func_1();
                                                  
                                                          
                                                          
                                                          
                                                          
                                                    
                                                    
                                                    
                                                      
                                                      
                                                      
                                                      
                                                      
                                                      
                                                      
                                                      
                                                      
                                                      
                                                      
                                                      
                                                            
                                                            
                                                            
                                                            
                                                      
                                                      
                           
     
                                
         
                                                                          
                                                                     
         
     
                           
     
                                
         
                                   
             
                                                                                    
                                                                                
             
         
     
                                                      
                           
     
                                                                
                                                          
     
                                                      
                           
     
                               
         
                                   
             
                                                                                    
                                                                                
             
         
     
                           
     
                                
         
                                                                          
                                                                     
         
     
                           
     
                                                                
                                                          
     
                           
     
                                                                  
                                                          
     
                                                        
                           
     
                                                                  
                                                          
     
                                                        
                                                        
                           
     
                                                                        
                                                                        
                                                                        
                                                                        
                                                          
     
                                                        
                                                        
                                                        
                           
     
                               
         
                                                                            
                                                                     
         
     
                           
     
                               
         
                                    
             
                                                                                      
                                                                                
             
         
     
                                                        
                                                        
                                                        
                                                        
                                                        
                                                        
                                                        
                                                                      
    return 0;
}
