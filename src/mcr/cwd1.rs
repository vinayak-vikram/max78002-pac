#[doc = "Register `CWD1` reader"]
pub type R = crate::R<Cwd1Spec>;
#[doc = "Register `CWD1` writer"]
pub type W = crate::W<Cwd1Spec>;
#[doc = "Field `data` reader - Code word Data0 the register retains its value while vregi supply present"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `data` writer - Code word Data0 the register retains its value while vregi supply present"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Code word Data0 the register retains its value while vregi supply present"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Code word Data0 the register retains its value while vregi supply present"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, Cwd1Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "Code Word Data1\n\nYou can [`read`](crate::Reg::read) this register and get [`cwd1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cwd1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cwd1Spec;
impl crate::RegisterSpec for Cwd1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cwd1::R`](R) reader structure"]
impl crate::Readable for Cwd1Spec {}
#[doc = "`write(|w| ..)` method takes [`cwd1::W`](W) writer structure"]
impl crate::Writable for Cwd1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CWD1 to value 0"]
impl crate::Resettable for Cwd1Spec {}
