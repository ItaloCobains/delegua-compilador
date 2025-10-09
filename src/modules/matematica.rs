use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::values::FunctionValue;
use inkwell::types::IntType;
use std::collections::HashMap;

pub struct Matematica<'ctx> {
    context: &'ctx Context,
    builder: Builder<'ctx>,
    i64_type: IntType<'ctx>,
}

impl<'ctx> Matematica<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        let builder = context.create_builder();
        let i64_type = context.i64_type();

        Matematica {
            context,
            builder,
            i64_type,
        }
    }

    pub fn declarar_funcoes(&self, module: &Module<'ctx>) -> HashMap<String, FunctionValue<'ctx>> {
        let mut funcoes = HashMap::new();

        let tipo_absoluto = self.i64_type.fn_type(&[self.i64_type.into()], false);
        let func_absoluto = module.add_function("matematica_absoluto", tipo_absoluto, None);
        funcoes.insert("absoluto".to_string(), func_absoluto);

        let tipo_potencia = self.i64_type.fn_type(&[self.i64_type.into(), self.i64_type.into()], false);
        let func_potencia = module.add_function("matematica_potencia", tipo_potencia, None);
        funcoes.insert("potencia".to_string(), func_potencia);

        let tipo_raiz = self.i64_type.fn_type(&[self.i64_type.into()], false);
        let func_raiz = module.add_function("matematica_raiz_quadrada", tipo_raiz, None);
        funcoes.insert("raiz_quadrada".to_string(), func_raiz);

        funcoes
    }

    pub fn gerar_implementacoes(&self, module: &Module<'ctx>) {
        self.gerar_absoluto(module);
        self.gerar_potencia(module);
        self.gerar_raiz_quadrada(module);
    }

    fn gerar_absoluto(&self, module: &Module<'ctx>) {
        let func_absoluto = module.get_function("matematica_absoluto").unwrap();
        let bloco_entrada = self.context.append_basic_block(func_absoluto, "entry");
        self.builder.position_at_end(bloco_entrada);

        let x = func_absoluto.get_nth_param(0).unwrap().into_int_value();

        let zero = self.i64_type.const_int(0, false);
        let eh_negativo = self.builder.build_int_compare(
            inkwell::IntPredicate::SLT,
            x,
            zero,
            "eh_negativo"
        ).unwrap();

        let menos_x = self.builder.build_int_neg(x, "menos_x").unwrap();
        let resultado = self.builder.build_select(eh_negativo, menos_x, x, "resultado_absoluto").unwrap();

        self.builder.build_return(Some(&resultado)).unwrap();
    }

    fn gerar_potencia(&self, module: &Module<'ctx>) {
        let func_potencia = module.get_function("matematica_potencia").unwrap();
        let bloco_entrada = self.context.append_basic_block(func_potencia, "entry");
        self.builder.position_at_end(bloco_entrada);

        let base = func_potencia.get_nth_param(0).unwrap().into_int_value();
        let expoente = func_potencia.get_nth_param(1).unwrap().into_int_value();

        let resultado_ptr = self.builder.build_alloca(self.i64_type, "resultado").unwrap();
        let contador_ptr = self.builder.build_alloca(self.i64_type, "contador").unwrap();

        let um = self.i64_type.const_int(1, false);
        let zero = self.i64_type.const_int(0, false);

        self.builder.build_store(resultado_ptr, um).unwrap();
        self.builder.build_store(contador_ptr, expoente).unwrap();

        let bloco_laco = self.context.append_basic_block(func_potencia, "laco");
        let bloco_corpo = self.context.append_basic_block(func_potencia, "corpo_laco");
        let bloco_apos = self.context.append_basic_block(func_potencia, "apos");

        self.builder.build_unconditional_branch(bloco_laco).unwrap();
        self.builder.position_at_end(bloco_laco);

        let contador = self.builder.build_load(self.i64_type, contador_ptr, "valor_contador").unwrap().into_int_value();
        let condicao = self.builder.build_int_compare(
            inkwell::IntPredicate::SGT,
            contador,
            zero,
            "condicao"
        ).unwrap();

        self.builder.build_conditional_branch(condicao, bloco_corpo, bloco_apos).unwrap();

        self.builder.position_at_end(bloco_corpo);
        let resultado = self.builder.build_load(self.i64_type, resultado_ptr, "valor_resultado").unwrap().into_int_value();
        let novo_resultado = self.builder.build_int_mul(resultado, base, "novo_resultado").unwrap();
        self.builder.build_store(resultado_ptr, novo_resultado).unwrap();

        let novo_contador = self.builder.build_int_sub(contador, um, "novo_contador").unwrap();
        self.builder.build_store(contador_ptr, novo_contador).unwrap();

        self.builder.build_unconditional_branch(bloco_laco).unwrap();

        self.builder.position_at_end(bloco_apos);
        let resultado_final = self.builder.build_load(self.i64_type, resultado_ptr, "resultado_final").unwrap();
        self.builder.build_return(Some(&resultado_final)).unwrap();
    }

    fn gerar_raiz_quadrada(&self, module: &Module<'ctx>) {
        let func_raiz = module.get_function("matematica_raiz_quadrada").unwrap();
        let bloco_entrada = self.context.append_basic_block(func_raiz, "entry");
        self.builder.position_at_end(bloco_entrada);

        let x = func_raiz.get_nth_param(0).unwrap().into_int_value();

        let zero = self.i64_type.const_int(0, false);
        let um = self.i64_type.const_int(1, false);

        let eh_zero_ou_um = self.builder.build_or(
            self.builder.build_int_compare(inkwell::IntPredicate::EQ, x, zero, "eh_zero").unwrap(),
            self.builder.build_int_compare(inkwell::IntPredicate::EQ, x, um, "eh_um").unwrap(),
            "eh_zero_ou_um"
        ).unwrap();

        let bloco_caso_especial = self.context.append_basic_block(func_raiz, "caso_especial");
        let bloco_caso_normal = self.context.append_basic_block(func_raiz, "caso_normal");
        let bloco_fim = self.context.append_basic_block(func_raiz, "fim");

        self.builder.build_conditional_branch(eh_zero_ou_um, bloco_caso_especial, bloco_caso_normal).unwrap();

        self.builder.position_at_end(bloco_caso_especial);
        self.builder.build_return(Some(&x)).unwrap();

        self.builder.position_at_end(bloco_caso_normal);
        let baixo_ptr = self.builder.build_alloca(self.i64_type, "baixo").unwrap();
        let alto_ptr = self.builder.build_alloca(self.i64_type, "alto").unwrap();
        let resultado_ptr = self.builder.build_alloca(self.i64_type, "resultado").unwrap();

        self.builder.build_store(baixo_ptr, um).unwrap();
        self.builder.build_store(alto_ptr, x).unwrap();

        let bloco_laco = self.context.append_basic_block(func_raiz, "laco");
        let bloco_corpo_laco = self.context.append_basic_block(func_raiz, "corpo_laco");
        let bloco_retornar_meio = self.context.append_basic_block(func_raiz, "retornar_meio");
        let bloco_apos_laco = self.context.append_basic_block(func_raiz, "apos_laco");

        self.builder.build_unconditional_branch(bloco_laco).unwrap();

        self.builder.position_at_end(bloco_laco);
        let baixo = self.builder.build_load(self.i64_type, baixo_ptr, "valor_baixo").unwrap().into_int_value();
        let alto = self.builder.build_load(self.i64_type, alto_ptr, "valor_alto").unwrap().into_int_value();

        let condicao = self.builder.build_int_compare(
            inkwell::IntPredicate::SLE,
            baixo,
            alto,
            "condicao"
        ).unwrap();

        self.builder.build_conditional_branch(condicao, bloco_corpo_laco, bloco_apos_laco).unwrap();

        self.builder.position_at_end(bloco_corpo_laco);
        let meio = self.builder.build_int_add(baixo, alto, "soma_meio").unwrap();
        let meio = self.builder.build_int_signed_div(meio, self.i64_type.const_int(2, false), "meio").unwrap();

        let meio_quadrado = self.builder.build_int_mul(meio, meio, "meio_quadrado").unwrap();
        
        let eh_exata = self.builder.build_int_compare(
            inkwell::IntPredicate::EQ,
            meio_quadrado,
            x,
            "eh_exata"
        ).unwrap();
        
        let bloco_continuar = self.context.append_basic_block(func_raiz, "continuar");
        self.builder.build_conditional_branch(eh_exata, bloco_retornar_meio, bloco_continuar).unwrap();
        
        self.builder.position_at_end(bloco_continuar);
        let comparacao = self.builder.build_int_compare(
            inkwell::IntPredicate::SGT,
            meio_quadrado,
            x,
            "comparacao"
        ).unwrap();

        let meio_menos_um = self.builder.build_int_sub(meio, um, "meio_menos_um").unwrap();
        let novo_alto = self.builder.build_select(comparacao, meio_menos_um, alto, "novo_alto").unwrap();
        let novo_baixo = self.builder.build_select(comparacao, baixo, self.builder.build_int_add(meio, um, "meio_mais_um").unwrap(), "novo_baixo").unwrap();

        self.builder.build_store(alto_ptr, novo_alto).unwrap();
        self.builder.build_store(baixo_ptr, novo_baixo).unwrap();
        self.builder.build_store(resultado_ptr, meio).unwrap();

        self.builder.build_unconditional_branch(bloco_laco).unwrap();

        self.builder.position_at_end(bloco_retornar_meio);
        self.builder.build_return(Some(&meio)).unwrap();

        self.builder.position_at_end(bloco_apos_laco);
        let resultado_final = self.builder.build_load(self.i64_type, resultado_ptr, "resultado_final").unwrap();
        self.builder.build_return(Some(&resultado_final)).unwrap();

        self.builder.position_at_end(bloco_fim);
        self.builder.build_unreachable().unwrap();
    }
}
