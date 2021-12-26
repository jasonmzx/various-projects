package net.fabricmc.rippedvillager;

import org.spongepowered.include.com.google.common.collect.ImmutableList;

import net.fabricmc.api.ModInitializer;
import net.fabricmc.fabric.api.event.player.AttackEntityCallback;
import net.fabricmc.fabric.api.item.v1.FabricItemSettings;
import net.fabricmc.fabric.api.object.builder.v1.block.FabricBlockSettings;
import net.fabricmc.fabric.api.object.builder.v1.entity.FabricDefaultAttributeRegistry;
import net.fabricmc.fabric.api.object.builder.v1.entity.FabricEntityTypeBuilder;
import net.minecraft.block.Block;
import net.minecraft.block.Material;
import net.minecraft.client.MinecraftClient;
import net.minecraft.client.model.ModelData;
import net.minecraft.client.model.ModelPart;
import net.minecraft.client.model.ModelPartBuilder;
import net.minecraft.client.model.*;
import net.minecraft.client.model.ModelPartData;
import net.minecraft.client.model.ModelTransform;
import net.minecraft.client.model.TexturedModelData;
import net.minecraft.client.render.VertexConsumer;
import net.minecraft.client.render.entity.EntityRendererFactory;
import net.minecraft.client.render.entity.MobEntityRenderer;
import net.minecraft.client.render.entity.model.EntityModel;
import net.minecraft.client.render.entity.model.EntityModelPartNames;
import net.minecraft.client.util.math.MatrixStack;
import net.minecraft.entity.EntityDimensions;
import net.minecraft.entity.EntityType;
import net.minecraft.entity.SpawnGroup;
import net.minecraft.entity.mob.PathAwareEntity;
import net.minecraft.entity.mob.MobEntity;
import net.minecraft.entity.mob.ZombieEntity;
import net.minecraft.entity.mob.HostileEntity;
import net.minecraft.entity.passive.VillagerEntity;
import net.minecraft.item.BlockItem;
import net.minecraft.item.ItemGroup;
import net.minecraft.network.MessageType;
import net.minecraft.text.Text;
import net.minecraft.util.ActionResult;
import net.minecraft.util.Identifier;
import net.minecraft.util.registry.Registry;
import net.minecraft.world.World;

public class RippedVillager implements ModInitializer {
	
	
    public static final Block EXAMPLE_BLOCK = new Block(FabricBlockSettings.of(Material.METAL).strength(4.0f));
	
    //Create and Register Entity:
    
    public static class StrongVillager extends PathAwareEntity {
    	
        public StrongVillager(EntityType<? extends PathAwareEntity> entityType, World world) {
            super(entityType, world);
        }
    }
    
    // Entity Type:
    
    public static final EntityType<StrongVillager> CUBE = Registry.register(
            Registry.ENTITY_TYPE,
            new Identifier("rippedvilly", "1"),
            FabricEntityTypeBuilder.create(SpawnGroup.MONSTER, StrongVillager::new).dimensions(EntityDimensions.fixed(1f, 3f)).build()
    );
    
    
    
    
    public static class StrongVillagerRenderer extends MobEntityRenderer<StrongVillager, StrongVillagerModel> {
    	 
        public StrongVillagerRenderer(EntityRendererFactory.Context context) {
            super(context, new StrongVillagerModel(context.getPart(EntityTestingClient.MODEL_CUBE_LAYER)), 0.5f);
        }
     
        @Override
        public Identifier getTexture(StrongVillager entity) {
            return new Identifier("rippedvillager", "texture.png");
        }
    }
    
    
    public static class StrongVillagerModel extends EntityModel<StrongVillager> {
   	 
        private final ModelPart base;
     
        public StrongVillagerModel(ModelPart modelPart) {
            this.base = modelPart.getChild(EntityModelPartNames.CUBE);
        }
        
        public static TexturedModelData getTexturedModelData() {
            ModelData modelData = new ModelData();
        	ModelPartData modelPartData = modelData.getRoot();
            modelPartData.addChild(EntityModelPartNames.CUBE, ModelPartBuilder.create()
            		//Legs:
            		.cuboid("bone", 2.0F, -16.0F, -6.0F, 4, 16, 4, 0, 56)	
            		.cuboid("bone", -8.0F, -16.0F, -6.0F, 4, 16, 4, 16 , 56)
            		
            		//Torso:
            		.cuboid("bone", -8.0F, -26.0F, -8.0F, 14, 10, 6, 30, 32)		
            		.cuboid("bone", -10.0F, -34.0F, -8.0F, 18, 8, 6, 0, 0)
            			//Pecs:
            		.cuboid("bone",-8.0F, -32.0F, -10.0F, 6 , 6 , 2, 60, 12)
            		.cuboid("bone",0.0F, -32.0F, -10.0F, 6 ,6 , 2, 28, 14)
            		
            		//Arms
            			//Deltoids
            		.cuboid("bone", 6.0F, -40.0F, -10.0F, 8, 8, 8, 0 ,40 )
            		.cuboid("bone", -16.0F, -40.0F, -10.0F, 8, 8, 8, 36, 14)	
            			
            			//Biceps
            		.cuboid("bone", -18.0F, -32.0F, -8.0F, 6, 6, 6, 32, 48)
            		.cuboid("bone", 10.0F, -32.0F, -8.0F, 6 , 6, 6 , 48, 0)
            			
            			//Forearms
            		.cuboid("bone", -18.0F, -28.0F, -6.0F, 4, 10, 4, 32, 60)
            		.cuboid("bone", 12.0F, -28.0F, -6.0F, 4, 10, 4, 56, 48)
            		
            		//head 
            		.cuboid("bone", -6.0F, -50.0F, -14.0F, 10, 16, 8, 0, 14)
            		.cuboid("bone", -2.0F, -44.0F, -16.0F, 2, 6, 2, 0, 14) //nose


            		
            		
            		
            		, ModelTransform.pivot(0F, 24F, 2F));
            
            return TexturedModelData.of(modelData, 128, 128);
        }
        
        @Override
        public void setAngles(StrongVillager entity, float limbAngle, float limbDistance, float animationProgress, float headYaw, float headPitch) {
        	
        }
        
        @Override
        public void render(MatrixStack matrices, VertexConsumer vertices, int light, int overlay, float red, float green, float blue, float alpha) {
            ImmutableList.of(this.base).forEach((modelRenderer) -> {
                modelRenderer.render(matrices, vertices, light, overlay, red, green, blue, alpha);
            });
        }

    }
    
    
    
	@Override
	public void onInitialize() {
		
		
        FabricDefaultAttributeRegistry.register(CUBE, StrongVillager.createMobAttributes());
		
		MinecraftClient mc = MinecraftClient.getInstance();
		
		
		AttackEntityCallback.EVENT.register( (player, world, hand, entity, hitResult) -> {

			mc.inGameHud.addChatMessage(MessageType.SYSTEM, Text.of("§6Hit!"), mc.player.getUuid());
			
			if(entity instanceof VillagerEntity ) {
				VillagerEntity selectedVillager = (VillagerEntity) entity;
				Float entityHP = selectedVillager.getHealth();
				
				
				if(entityHP <= 12) {
					entity.discard();
					var strongVillager = CUBE.create(world);
					strongVillager.refreshPositionAndAngles(entity.getX(),entity.getY(), entity.getZ(), entity.getYaw(), 0.0F);
					world.spawnEntity(strongVillager);
				}
				
				mc.inGameHud.addChatMessage(MessageType.SYSTEM, Text.of(String.valueOf(entityHP)), mc.player.getUuid());
				
			}
			
			
			return ActionResult.PASS;
		});
		
		
        Registry.register(Registry.BLOCK, new Identifier("tutorial", "example_block"), EXAMPLE_BLOCK);
        Registry.register(Registry.ITEM, new Identifier("tutorial", "example_block"), new BlockItem(EXAMPLE_BLOCK, new FabricItemSettings().group(ItemGroup.MISC)));
	}

}
